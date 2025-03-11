use super::enums::{AddTextLineToContainer, AddToggleLineToContainer};
use super::{
    app::App,
    enums::{UiInputLineAppState, UiInputLineValue, UiLineLabel, UiTextInputLine, UiToggleLine},
};

use ratatui::{
    layout::{Constraint, Layout, Rect},
    prelude::{Backend, Direction},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Wrap},
    Frame,
};

/// Renders the input blade UI component.
///
/// This function constructs a list of `Line` elements representing various input fields and toggleable
/// options. It visually highlights the currently focused input, providing a smooth user experience.
///
/// # Arguments
///
/// * `frame` - The terminal frame where the UI component is rendered.
/// * `area` - The rectangular region within the frame where the component is drawn.
///
/// # UI Structure
///
/// The rendered component consists of:
/// - A bordered block titled "Azure Configuration"
/// - Editable input fields (e.g., Application Name, Owner's Email)
/// - Toggleable options for environment selection (e.g., Dev, Test, Stage, Prod)
/// - Additional resource options (e.g., Database, Cache, Storage)
pub fn render_input_blade<B: Backend>(app_state: &App, frame: &mut Frame, area: Rect) {
    let mut ui_input_lines: Vec<Line> = Vec::new();

    UiTextInputLine::new()
        .app_state(app_state)
        .label("Application Name: ")
        .value(&app_state.config.resource_name)
        .container(&mut ui_input_lines);

    UiTextInputLine::new()
        .app_state(app_state)
        .label("Owner's Email: ")
        .value(&app_state.config.owner_email)
        .container(&mut ui_input_lines);
    // .index(2);

    let env_header = Line::from(vec![Span::styled(
        "  Environments:",
        Style::default()
            .fg(Color::Rgb(45, 152, 218))
            .add_modifier(Modifier::BOLD),
    )]);
    ui_input_lines.push(env_header);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Dev")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Test")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Stage")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Prod")
        .container(&mut ui_input_lines);

    let resources_header = Line::from(vec![Span::styled(
        "  Additional Resources:",
        Style::default()
            .fg(Color::Rgb(45, 152, 218))
            .add_modifier(Modifier::BOLD),
    )]);
    ui_input_lines.push(resources_header);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure SQL")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure Cache for Redis")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure Service Bus")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure Storage")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure Key Vault")
        .container(&mut ui_input_lines);

    UiToggleLine::new()
        .app_state(app_state)
        .label("Azure Container Registry")
        .container(&mut ui_input_lines);

    let input_widget = Paragraph::new(ui_input_lines).block(
        Block::default()
            .title(Span::styled(
                "Azure Configuration",
                Style::default()
                    .fg(Color::Rgb(74, 105, 189))
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(112, 161, 255))),
    );

    frame.render_widget(input_widget, area);
}

pub fn render_config_review<B: Backend>(app_state: &App, frame: &mut Frame, area: Rect) {
    let config_json = serde_json::to_string_pretty(&app_state.config)
        .unwrap_or_else(|_| "Invalid JSON".to_string());
    let styled_json = styled_json(&config_json);

    let review = Paragraph::new(styled_json).block(
        Block::default()
            .title("Config Review (JSON)")
            .style(Style::default().fg(Color::Rgb(255, 127, 80)))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(112, 161, 255))),
    );

    frame.render_widget(review, area);
}

fn styled_json(json: &str) -> Vec<Line<'static>> {
    let mut styled_lines = Vec::new();

    for line in json.lines() {
        let mut spans = Vec::new();

        if line.contains(':') {
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            let key = parts[0];
            let value = parts[1];

            spans.push(Span::styled(
                key.to_string(),
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            ));

            spans.push(Span::raw(":"));
            spans.push(Span::styled(
                value.to_string(),
                Style::default().fg(Color::Rgb(255, 127, 80)),
            ));
        } else {
            spans.push(Span::raw(line.to_string()));
        }

        styled_lines.push(Line::from(spans));
    }
    styled_lines
}

pub fn render_help_context<B: Backend>(
    app_state: &App,
    frame: &mut Frame,
    area: Rect,
    error_message: Option<String>,
) {
    let help_text = match app_state.focus {
        0 => {
            "Enter the application name.\n\n\
            This will be used as a base name for all resources.\n\
            Typically you would use some convention that I have to find out about."
        }
        1 => {
            "Provide the owner's email.\n\n\
            Notifications/Alerts will be sent here.\n\
            This is typically a project lead or other. I need to confirm."
        }
        3..=6 => {
            "Select the environments where the application will be deployed.\n\n\
            Each environment maps to a specific Azure Subscription by default.\n\n\
            Dev = dev-01\n\
            Test = non-prod\n\
            Stage = preprod\n\
            Prod = prod-01"
        }
        8 => "Include Azure SQL resources in the project.\n\n\
            - Azure SQL Server.\n\
            - Azure SQL Database.\n\
            - Managed relational database service for SQL Server.",
        9 => "Include Azure Cache for Redis resource in the project.\n\n\
            - Azure Cache for Redis.\n\
            - In-memory data structure store, used as a database, cache, and message broker.",
        10 => "Include Azure Service Bus resource in the project.\n\n\
            - Azure Service Bus.\n\
            - Fully managed enterprise message broker with message queues and publish-subscribe topics.",
        11 => "Include Azure Storage resource in the project.\n\n\
            - Azure Storage Account.\n\
            - Durable, highly available, and massively scalable cloud storage solution.",
        12 => "Include Azure Key Vault resource in the project.\n\n\
            - Azure Key Vault.\n\
            - Used for storing secrets, keys, and certificates securely.",
        13 => " - Azure Container Registry.\n\
                - Used for storing and managing container images.",
        _ => "Use arrow keys to navigate. Press 'Enter' to confirm selections.",
    };

    let help_lines: Vec<Line> = help_text
        .lines()
        .map(|line| {
            Line::from(Span::raw(line).style(Style::default().fg(Color::Rgb(189, 197, 129))))
        })
        .collect();

    let error_display = error_message.unwrap_or_else(|| String::new());

    let mut key_actions = vec![
        Line::from(vec![Span::styled(
            "Controls",
            Style::default()
                .fg(Color::Rgb(120, 111, 166))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::raw("\n")),
        Line::from(Span::raw("  ↑/↓ - Navigate through options."))
            .style(Style::default().fg(Color::LightYellow)),
        Line::from(Span::raw("  ➔  - Toggle all of current list."))
            .style(Style::default().fg(Color::LightYellow)),
        Line::from(Span::raw(
            "  Spacebar - Toggle selection of current option.",
        ))
        .style(Style::default().fg(Color::LightYellow)),
        Line::from(Span::raw(
            "  Enter - Confirm selection & generate the Pulumi project.",
        ))
        .style(Style::default().fg(Color::LightYellow)),
        Line::from(Span::raw("\n")),
        Line::from(vec![Span::styled(
            "Focus Help",
            Style::default()
                .fg(Color::Rgb(120, 111, 166))
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(Span::raw("\n")),
    ];

    key_actions.extend(help_lines);

    let validation_handle = if !error_display.is_empty() {
        Line::from(vec![
            Span::styled(
                r#"⚠  "#,
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(error_display).style(Style::default().fg(Color::LightRed)),
        ])
    } else {
        Line::from(Span::raw(""))
    };

    key_actions.push(Line::from(Span::raw("\n")));
    key_actions.push(validation_handle);

    let help_paragraph = Paragraph::new(key_actions)
        .block(
            Block::default()
                .title(Span::styled(
                    "Help & Guidance",
                    Style::default()
                        .fg(Color::Rgb(136, 84, 208))
                        .add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL)
                .padding(Padding::horizontal(5))
                .padding(Padding::vertical(1))
                .border_style(Style::default().fg(Color::Rgb(112, 161, 255))),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(help_paragraph, area);
}

pub fn render_exit_screen<B: Backend>(frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.area());

    let text = vec![
        Line::from("Exit Screen - Operation Complete!"),
        Line::from("Everything has been successfully processed."),
        Line::from("Press any key to exit."),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Success")
        .style(Style::default().fg(Color::Green));

    let exit_chunk = Paragraph::new(text).block(block);

    frame.render_widget(exit_chunk, chunks[0]);
}
