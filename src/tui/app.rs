use std::{io, vec};

use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Backend,
    Terminal,
};
use serde::{Deserialize, Serialize};

use super::{
    constants::{ENVIRONMENT_OPTIONS, RESOURCE_OPTIONS, SELECTION_OFFSET},
    enums::{AppEnvironment, ResourceType},
    ui::{self},
    validation::{self},
};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ProjectConfig {
    pub resource_name: String,
    pub environments: Vec<String>,
    pub additional_resources: Vec<String>,
    pub owner_email: String,
}

impl ProjectConfig {
    pub fn default(name: &str) -> Self {
        Self {
            resource_name: name.to_owned(),
            environments: vec![
                AppEnvironment::Dev.to_string().to_lowercase(),
                AppEnvironment::Test.to_string().to_lowercase(),
                AppEnvironment::Stage.to_string().to_lowercase(),
                AppEnvironment::Prod.to_string().to_lowercase(),
            ],
            additional_resources: vec![
                ResourceType::Database.to_string().to_lowercase(),
                ResourceType::Cache.to_string().to_lowercase(),
                ResourceType::ServiceBus.to_string().to_lowercase(),
                ResourceType::Storage.to_string().to_lowercase(),
                ResourceType::Keyvault.to_string().to_lowercase(),
                ResourceType::ContainerRegistry.to_string().to_lowercase(),
            ],
            owner_email: String::new(),
        }
    }
}

pub struct App {
    pub exit: bool,
    pub confirmation: bool,
    _input_buffer: String,
    pub config: ProjectConfig,
    pub focus: usize, // Index of the currently focused input field
    pub user_input_container: Vec<String>,
    pub validation_error: Option<String>,
    pub show_exit_screen: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            confirmation: false,
            _input_buffer: String::new(),
            config: ProjectConfig::default("default_name"),
            focus: 0, // Start focus on the first input field
            user_input_container: Vec::new(),
            validation_error: None,
            show_exit_screen: false,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        while !self.exit {
            // Read user input (key event) and handle it
            match crossterm::event::read()? {
                Event::Key(key_event) => self.handle_key_event(key_event)?,
                _ => &App::new(), // This seems unnecessary as it's just a placeholder; can be removed
            };

            // Draw the UI on each iteration, passing the error message if validation failed
            terminal.draw(|frame| {
                if let Some(error_message) = &self.validation_error {
                    // Pass the error message to the draw method if there was a validation error
                    self.draw::<B>(frame, Some(error_message.clone()));
                } else {
                    // No error, pass None
                    self.draw::<B>(frame, None);
                }
            })?;

            // If the user presses Enter, perform validation and confirm selections
            if self.confirmation {
                // Trigger validation only when Enter is pressed, validation is in `confirm_selections()`
                if let Err(error_message) = validation::validate_inputs(&self) {
                    // If validation fails, reset confirmation and display an error message
                    self.confirmation = false;
                    self.validation_error = Some(error_message); // Store the error message
                } else {
                    // If validation passes, set confirmation to true and clear the error message
                    self.confirmation = true;
                    self.validation_error = None; // Clear the error message if validation passes
                    self.exit = true;
                }
            }
        }

        self.show_exit_screen = true;
        terminal.clear()?;

        Ok(())
    }

    fn draw<B: Backend>(&self, frame: &mut ratatui::Frame, err_message: Option<String>) {
        // Divide the screen into two main columns
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Percentage(50), // Input Blade (left)
                    Constraint::Percentage(50), // Review and Chart (right)
                ]
                .as_ref(),
            )
            .split(frame.area());

        // Divide the right column into two rows: Config Review and Chart View
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Percentage(70), // JSON/YAML Review
                    Constraint::Percentage(30), // Chart View
                ]
                .as_ref(),
            )
            .split(chunks[1]);

        if self.show_exit_screen {
            // Render the Exit Screen here
            ui::render_exit_screen::<B>(frame);
        } else {
            // Render the Input Blade
            ui::render_input_blade::<B>(self, frame, chunks[0]);

            // Render the Help Context, passing the error message if present
            ui::render_help_context::<B>(self, frame, right_chunks[0], err_message);

            // Render the Config Review
            ui::render_config_review::<B>(self, frame, right_chunks[1]);
        }
    }

    fn handle_keycode_right_env(&mut self, environment: AppEnvironment) {
        if self
            .config
            .environments
            .contains(&environment.to_string().to_lowercase())
        {
            self.config
                .environments
                .retain(|env| env != &environment.to_string().to_lowercase());
        } else {
            self.config
                .environments
                .push(environment.to_string().to_lowercase())
        }
    }

    fn handle_keycode_right_resource(&mut self, resource: ResourceType) {
        if self
            .config
            .additional_resources
            .contains(&resource.to_string().to_lowercase())
        {
            self.config
                .additional_resources
                .retain(|res| res != &resource.to_string().to_lowercase());
        } else {
            self.config
                .additional_resources
                .push(resource.to_string().to_lowercase())
        }
    }

    fn environments_toggle_all(&mut self) {
        let all_selected: bool = ENVIRONMENT_OPTIONS.iter().all(|env| {
            self.config
                .environments
                .contains(&env.to_string().to_lowercase())
        });

        if all_selected {
            self.config.environments = Vec::new();
        } else {
            for env in ENVIRONMENT_OPTIONS {
                let env_str = env.to_string().to_lowercase();
                if !self.config.environments.contains(&env_str) {
                    self.config.environments.push(env_str);
                }
            }
        }
    }

    fn resources_toggle_all(&mut self) {
        let all_selected: bool = RESOURCE_OPTIONS.iter().all(|resource| {
            self.config
                .additional_resources
                .contains(&resource.to_string().to_lowercase())
        });

        if all_selected {
            self.config.additional_resources = Vec::new();
        } else {
            for resource in RESOURCE_OPTIONS {
                let resource_str = resource.to_string().to_lowercase();
                if !self.config.additional_resources.contains(&resource_str) {
                    self.config.additional_resources.push(resource_str);
                }
            }
        }
    }

    fn cycle_focus(&mut self, direction: isize) {
        let total_options = ENVIRONMENT_OPTIONS.len() + RESOURCE_OPTIONS.len() + SELECTION_OFFSET;
        self.focus = ((self.focus as isize + direction + total_options as isize)
            % total_options as isize) as usize;
        if self.focus == 2 || self.focus == 7 {
            self.focus = (self.focus as isize + direction) as usize;
        }
    }

    fn handle_key_event(&mut self, key_event: crossterm::event::KeyEvent) -> io::Result<&App> {
        if key_event.kind == KeyEventKind::Press {
            match key_event.code {
                KeyCode::Esc => {
                    self.exit = true;
                }

                KeyCode::Down | KeyCode::Tab => {
                    self.cycle_focus(1);
                }

                KeyCode::Up => {
                    self.cycle_focus(-1);
                }

                KeyCode::Right => match self.focus {
                    3 | 4 | 5 | 6 => self.environments_toggle_all(),
                    8 | 9 | 10 | 11 | 12 | 13 => self.resources_toggle_all(),
                    _ => (),
                },

                KeyCode::Char(c) => match self.focus {
                    0 => self.config.resource_name.push(c),
                    1 => self.config.owner_email.push(c),
                    3 => self.handle_keycode_right_env(AppEnvironment::Dev),
                    4 => self.handle_keycode_right_env(AppEnvironment::Test),
                    5 => self.handle_keycode_right_env(AppEnvironment::Stage),
                    6 => self.handle_keycode_right_env(AppEnvironment::Prod),
                    8 => self.handle_keycode_right_resource(ResourceType::Database),
                    9 => self.handle_keycode_right_resource(ResourceType::Cache),
                    10 => self.handle_keycode_right_resource(ResourceType::ServiceBus),
                    11 => self.handle_keycode_right_resource(ResourceType::Storage),
                    12 => self.handle_keycode_right_resource(ResourceType::Keyvault),
                    13 => self.handle_keycode_right_resource(ResourceType::ContainerRegistry),
                    _ => {}
                },

                KeyCode::Backspace => match self.focus {
                    0 => {
                        self.config.resource_name.pop();
                    }
                    1 => {
                        self.config.owner_email.pop();
                    }
                    _ => {}
                },

                KeyCode::Enter => self.confirm_selections()?,

                // Implementing Delete key behavior:
                KeyCode::Delete => match self.focus {
                    0 => {
                        self.config.resource_name.clear();
                    }
                    1 => {
                        self.config.owner_email.clear();
                    }
                    _ => {}
                },

                _ => {} // Handle other keys if needed
            }
        }
        Ok(self)
    }

    pub fn confirm_selections(&mut self) -> io::Result<()> {
        if let Err(error_message) = validation::validate_inputs(&self) {
            self.confirmation = false;

            // Store the error message in the App state
            self.validation_error = Some(error_message.clone()); // Save error message

            return Ok(()); // Stay in the same state, don't exit
        }

        // If validation passes, set confirmation to true
        self.confirmation = true;
        self.validation_error = None; // Clear any previous error if validation is successful

        Ok(())
    }
}
