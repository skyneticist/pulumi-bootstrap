use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use strum::Display;

use crate::tui::app::App;

pub struct UiTextInputLine<'a, 'b> {
    pub app_state: Option<&'b App>,
    pub label: &'a str,
    pub value: &'a str,
    pub index: usize,
}

impl<'a, 'b> UiTextInputLine<'a, 'b> {
    pub fn new() -> Self {
        Self {
            app_state: None,
            label: "Label",
            value: "Value",
            index: 0,
        }
    }
}

pub trait AddTextLineToContainer<'a, 'b> {
    fn container(&mut self, container: &mut Vec<Line<'a>>) -> &mut UiTextInputLine<'a, 'b>;
}

impl<'a, 'b> AddTextLineToContainer<'a, 'b> for UiTextInputLine<'a, 'b> {
    fn container(&mut self, container: &mut Vec<Line<'a>>) -> &mut UiTextInputLine<'a, 'b> {
        self.index = container.len();
        let focused_style: Style = Style::default()
            .fg(Color::Rgb(255, 99, 72))
            .add_modifier(Modifier::BOLD);

        let normal_style: Style = Style::default().fg(Color::Rgb(164, 176, 190));

        let is_focused = self.app_state.unwrap().focus == self.index;
        let style = if is_focused {
            focused_style
        } else {
            normal_style
        };

        let line = Line::from(vec![
            Span::styled(if is_focused { "> " } else { "  " }, style),
            Span::styled(self.label, style),
            Span::raw(self.value),
        ]);
        container.push(line);
        self
    }
}

pub trait FocusIndex {
    fn index(&mut self, index: usize) -> &mut Self;
}

impl<'a, 'b> FocusIndex for UiTextInputLine<'a, 'b> {
    fn index(&mut self, index: usize) -> &mut Self {
        self.index = index;
        self
    }
}

pub trait UiInputLineAppState<'a> {
    fn app_state(&mut self, app_state: &'a App) -> &mut Self;
}

impl<'a, 'b: 'a> UiInputLineAppState<'b> for UiTextInputLine<'a, 'b> {
    fn app_state(&mut self, app_state: &'b App) -> &mut Self {
        self.app_state = Some(app_state);
        self
    }
}

pub trait UiLineLabel<'a> {
    fn label(&mut self, label: &'a str) -> &mut Self;
}

impl<'a, 'b> UiLineLabel<'a> for UiTextInputLine<'a, 'b> {
    fn label(&mut self, label: &'a str) -> &mut Self {
        self.label = label;
        self
    }
}

pub trait UiInputLineValue<'a> {
    fn value(&mut self, value: &'a str) -> &mut Self;
}

impl<'a, 'b> UiInputLineValue<'a> for UiTextInputLine<'a, 'b> {
    fn value(&mut self, value: &'a str) -> &mut Self {
        self.value = value;
        self
    }
}

pub struct UiToggleLine<'a, 'b> {
    pub app_state: Option<&'b App>,
    pub label: &'a str,
    pub index: usize,
}

pub trait AddToggleLineToContainer<'a, 'b> {
    fn container(&mut self, container: &mut Vec<Line<'a>>) -> &mut UiToggleLine<'a, 'b>;
}

impl<'a, 'b> AddToggleLineToContainer<'a, 'b> for UiToggleLine<'a, 'b> {
    fn container(&mut self, container: &mut Vec<Line<'a>>) -> &mut UiToggleLine<'a, 'b> {
        self.index = container.len(); //
        let focused_style: Style = Style::default()
            .fg(Color::Rgb(255, 99, 72))
            .add_modifier(Modifier::BOLD);

        let normal_style: Style = Style::default().fg(Color::Rgb(164, 176, 190));
        let is_focused = self.app_state.unwrap().focus == self.index;
        let style = if is_focused {
            focused_style
        } else {
            normal_style
        };

        let line = Line::from(vec![
            // Span::styled(self.label, style),
            Span::raw(format!(
                "{}{}",
                if self.app_state.unwrap().focus == self.index {
                    "> "
                } else {
                    "  "
                },
                if self
                    .app_state
                    .unwrap()
                    .config
                    .environments
                    .contains(&self.label.to_lowercase())
                    || self
                        .app_state
                        .unwrap()
                        .config
                        .additional_resources
                        .contains(&self.label.to_lowercase())
                {
                    "[x] "
                } else {
                    "[ ] "
                },
            )), // Span::raw("Dev"),
            Span::styled(self.label, style),
        ]);
        container.push(line);
        self
    }
}

impl<'a, 'b> UiToggleLine<'a, 'b> {
    pub fn new() -> Self {
        Self {
            app_state: None,
            label: "Label",
            index: 0,
        }
    }
}

impl<'a, 'b> FocusIndex for UiToggleLine<'a, 'b> {
    fn index(&mut self, index: usize) -> &mut Self {
        self.index = index;
        self
    }
}

impl<'a, 'b: 'a> UiInputLineAppState<'b> for UiToggleLine<'a, 'b> {
    fn app_state(&mut self, app_state: &'b App) -> &mut Self {
        self.app_state = Some(app_state);
        self
    }
}

impl<'a, 'b> UiLineLabel<'a> for UiToggleLine<'a, 'b> {
    fn label(&mut self, label: &'a str) -> &mut Self {
        self.label = label;
        self
    }
}

// pub struct UiBuilder<'a> {
//     app_state: &'a App,
//     lines: Vec<Line<'a>>,
// }
// impl<'a> UiBuilder<'a> {
//     pub fn new(app_state: &'a App) -> Self {
//         UiBuilder {
//             app_state,
//             lines: Vec::new(),
//         }
//     }
//     pub fn add_input(
//         &mut self,
//         label: &'a str,
//         value: &'a str,
//         index: usize,
//     ) -> &mut UiBuilder<'a> {
//         let mut input = UiTextInputLine::new();
//         // UiTextInputLine::new().app_state(&self.app_state).label(label).value(value).index(index);
//         input.app_state(&self.app_state).label(label).value(value).index(index);
//         self.lines.push(
//             input.to_styled_line()
//                 // .app_state(&self.app_state)
//                 // .label(label)
//                 // .value(value)
//                 // .index(index)
//                 // .to_styled_line(),
//         );
//         self
//     }
//     pub fn add_toggle(mut self, label: &'a str, index: usize) -> Self {
//         let toggle = UiToggleLine::new()
//             .app_state(self.app_state)
//             .label(label)
//             .index(index);
//         self.lines.push(toggle.to_styled_line());
//         self
//     }
//     pub fn add_label(mut self, label: &'a str, color: Color) -> Self {
//         let label_line = Line::from(vec![Span::styled(
//             label,
//             Style::default().fg(color).add_modifier(Modifier::BOLD),
//         )]);
//         self.lines.push(label_line);
//         self
//     }
//     pub fn build(self) -> Vec<Line<'a>> {
//         self.lines
//     }
// }

#[derive(Display, Debug, PartialEq)]
pub enum AppEnvironment {
    Dev = 3,
    Prod = 6,
    Stage = 5,
    Test = 4,
}

#[derive(Clone, Display, Debug, PartialEq)]
pub enum ResourceType {
    Database = 8,
    Cache = 9,
    ServiceBus = 10,
    Storage = 11,
    Keyvault = 12,
    ContainerRegistry = 13,
}

#[derive(Debug)]
pub struct TextInputFieldProperties {
    pub label: &'static str,
    pub value: &'static str,
}

pub trait SetUserOption {
    fn set_user_option(&self, app_state: &App) -> String;
}

impl SetUserOption for AppEnvironment {
    fn set_user_option(&self, app_state: &App) -> String {
        format!(
            "{}{} {}",
            if app_state.focus == self.value() {
                "> "
            } else {
                "  "
            },
            if app_state
                .config
                .environments
                .contains(&self.to_string().to_lowercase())
            {
                "[x]"
            } else {
                "[ ]"
            },
            self.to_string().to_lowercase()
        )
    }
}

impl SetUserOption for ResourceType {
    fn set_user_option(&self, app_state: &App) -> String {
        format!(
            "{}{} {}",
            if app_state.focus == self.value() {
                "> "
            } else {
                "  "
            },
            if app_state
                .config
                .additional_resources
                .contains(&self.to_string().to_lowercase())
            {
                "[x]"
            } else {
                "[ ]"
            },
            self.to_string().to_lowercase()
        )
    }
}

// impl SetUserOption for TextInputFields {
//     fn set_user_option(&self, app_state: &App) -> String {
//         match self {
//             TextInputFields::AppName(_) => {
//                 format!(
//                     "{}{} {}",
//                     if app_state.focus == self.value() {
//                         "> "
//                     } else {
//                         "  "
//                     },
//                     "App Name",
//                     app_state.config.resource_name
//                 )
//             }
//             TextInputFields::OwnerEmail(_) => {
//                 format!(
//                     "{}{} {}",
//                     if app_state.focus == self.value() {
//                         "> "
//                     } else {
//                         "  "
//                     },
//                     "Owner Email",
//                     app_state.config.owner_email
//                 )
//             }
//         }
//     }
// }

pub trait EnumIndex {
    fn value(&self) -> usize;
}

impl EnumIndex for AppEnvironment {
    fn value(&self) -> usize {
        match self {
            AppEnvironment::Dev => 3,
            AppEnvironment::Test => 4,
            AppEnvironment::Stage => 5,
            AppEnvironment::Prod => 6,
        }
    }
}

impl EnumIndex for ResourceType {
    fn value(&self) -> usize {
        match self {
            ResourceType::Database => 8,
            ResourceType::Cache => 9,
            ResourceType::ServiceBus => 10,
            ResourceType::Storage => 11,
            ResourceType::Keyvault => 12,
            ResourceType::ContainerRegistry => 13,
        }
    }
}
