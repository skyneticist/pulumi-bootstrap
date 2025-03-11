use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table, TableState},
    Terminal,
};
use std::io;
use tokio;

#[derive(Debug)]
struct Resource {
    name: String,
    description: String,
    instructions: String,
}

impl Resource {
    fn new(name: &str, description: &str, instructions: &str) -> Self {
        Resource {
            name: name.to_string(),
            description: description.to_string(),
            instructions: instructions.to_string(),
        }
    }
}

async fn run() -> Result<(), io::Error> {
    let resources = vec![
        Resource::new("keyvault", "Azure Key Vault for secure secrets", "Configure secret storage"),
        Resource::new("storageaccount", "Azure Storage Account for blob storage", "Set up storage containers"),
        Resource::new("appservice", "Azure App Service for hosting web apps", "Deploy web applications"),
        Resource::new("cosmosdb", "Azure Cosmos DB for globally distributed databases", "Create a new database"),
        Resource::new("vnet", "Azure Virtual Network for network isolation", "Set up virtual networks"),
        Resource::new("functionapp", "Azure Function App for serverless computing", "Deploy serverless functions"),
    ];

    // Setup terminal with Ratatui
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // Define the table layout and data
    let table = Table::new(resources.iter().map(|r| {
        Row::new(vec![
            Cell::from(r.name.as_str()).style(Style::default().fg(Color::White)),
            Cell::from(r.description.as_str()).style(Style::default().fg(Color::Gray)),
            Cell::from(r.instructions.as_str()).style(Style::default().fg(Color::Cyan)),
        ])
    }))
    .header(Row::new(vec![
        Cell::from("Resource Type").style(Style::default().fg(Color::Yellow)),
        Cell::from("Description").style(Style::default().fg(Color::Yellow)),
        Cell::from("Instructions").style(Style::default().fg(Color::Yellow)),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Azure Resources"))
    .widths(&[Constraint::Percentage(40), Constraint::Percentage(40), Constraint::Percentage(20)]);

    // Define table state for selecting
    let mut state = TableState::default();
    state.select(Some(0)); // Start with the first row selected

    // Main event loop to render and interact with the table
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            f.render_stateful_widget(table.clone(), chunks[0], &mut state);
        })?;

        // Handle user input for selection
        if let Some(key) = crossterm::event::read()? {
            match key {
                crossterm::event::Event::Key(crossterm::event::KeyEvent {
                    code,
                    modifiers: _,
                    kind: _,
                    state: _,
                }) => match code {
                    crossterm::event::KeyCode::Down => {
                        if let Some(i) = state.selected() {
                            if i < resources.len() - 1 {
                                state.select(Some(i + 1));
                            }
                        }
                    }
                    crossterm::event::KeyCode::Up => {
                        if let Some(i) = state.selected() {
                            if i > 0 {
                                state.select(Some(i - 1));
                            }
                        }
                    }
                    crossterm::event::KeyCode::Enter => {
                        if let Some(i) = state.selected() {
                            let selected_resource = &resources[i];
                            println!("\nYou selected: {} - {}", selected_resource.name, selected_resource.instructions);
                            break; // Exit after selection
                        }
                    }
                    crossterm::event::KeyCode::Esc => {
                        break; // Exit on Escape
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}

// #[tokio::main]
// async fn main() {
//     if let Err(e) = run().await {
//         eprintln!("Error: {}", e);
//     }
// }
