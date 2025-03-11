// // use ratatui::{
// //     backend::CrosstermBackend,
// //     layout::{Constraint, Direction, Layout},
// //     style::{Color, Style},
// //     widgets::{Block, Borders, Cell, Row, Table, TableState},
// //     Terminal,
// // };
// // use std::io;

// // #[derive(Debug)]
// // struct Resource {
// //     name: String,
// //     description: String,
// //     instructions: String,
// // }

// // impl Resource {
// //     fn new(name: &str, description: &str, instructions: &str) -> Self {
// //         Resource {
// //             name: name.to_string(),
// //             description: description.to_string(),
// //             instructions: instructions.to_string(),
// //         }
// //     }
// // }

// // pub async fn run_table() -> Result<(), io::Error> {
// //     let resources = vec![
// //         Resource::new(
// //             "keyvault",
// //             "Azure Key Vault for secure secrets",
// //             "Configure secret storage",
// //         ),
// //         Resource::new(
// //             "storageaccount",
// //             "Azure Storage Account for blob storage",
// //             "Set up storage containers",
// //         ),
// //         Resource::new(
// //             "appservice",
// //             "Azure App Service for hosting web apps",
// //             "Deploy web applications",
// //         ),
// //         Resource::new(
// //             "cosmosdb",
// //             "Azure Cosmos DB for globally distributed databases",
// //             "Create a new database",
// //         ),
// //         Resource::new(
// //             "vnet",
// //             "Azure Virtual Network for network isolation",
// //             "Set up virtual networks",
// //         ),
// //         Resource::new(
// //             "functionapp",
// //             "Azure Function App for serverless computing",
// //             "Deploy serverless functions",
// //         ),
// //     ];

// //     // Setup terminal with Ratatui
// //     let stdout = io::stdout();
// //     let backend = CrosstermBackend::new(stdout);
// //     let mut terminal = Terminal::new(backend)?;

// //     terminal.clear()?;

// //     // Define the table layout and data
// //     let table = Table::new(
// //         resources.iter().map(|r| {
// //             Row::new(vec![
// //                 Cell::from(r.name.as_str()).style(Style::default().fg(Color::White)),
// //                 Cell::from(r.description.as_str()).style(Style::default().fg(Color::Gray)),
// //                 Cell::from(r.instructions.as_str()).style(Style::default().fg(Color::Cyan)),
// //             ])
// //         }),
// //         &[
// //             Constraint::Percentage(25),
// //             Constraint::Percentage(35),
// //             Constraint::Percentage(40),
// //         ],
// //     )
// //     .header(Row::new(vec![
// //         Cell::from("Resource Type").style(Style::default().fg(Color::Yellow)),
// //         Cell::from("Description").style(Style::default().fg(Color::Yellow)),
// //         Cell::from("Instructions").style(Style::default().fg(Color::Yellow)),
// //     ]))
// //     .block(
// //         Block::default()
// //             .borders(Borders::ALL)
// //             .title("Azure Resources"),
// //     )
// //     .widths(&[
// //         Constraint::Percentage(25),
// //         Constraint::Percentage(35),
// //         Constraint::Percentage(40),
// //     ]);

// //     // Define table state for selecting
// //     let mut state = TableState::default().with_selected(0);
// //     // state.select(Some(0)); // Start with the first row selected

// //     // Main event loop to render and interact with the table
// //     loop {
// //         terminal.draw(|f| {
// //             let size = f.area();
// //             let chunks = Layout::default()
// //                 .direction(Direction::Vertical)
// //                 .constraints([Constraint::Percentage(100)].as_ref())
// //                 .split(size);

// //             f.render_stateful_widget(table.clone(), chunks[0], &mut state);
// //         })?;

// //         // Handle user input for selection
// //         if let crossterm::event::Event::Key(crossterm::event::KeyEvent { code, .. }) =
// //             crossterm::event::read()?
// //         {
// //             match code {
// //                 crossterm::event::KeyCode::Down => {
// //                     if let Some(i) = state.selected() {
// //                         if i < resources.len() - 1 {
// //                             state.select(Some(i + 1));
// //                         }
// //                     }
// //                 }
// //                 crossterm::event::KeyCode::Up => {
// //                     if let Some(i) = state.selected() {
// //                         if i > 0 {
// //                             state.select(Some(i - 1));
// //                         }
// //                     }
// //                 }
// //                 crossterm::event::KeyCode::Enter => {
// //                     if let Some(i) = state.selected() {
// //                         let selected_resource = &resources[i];
// //                         println!(
// //                             "\nYou selected: {} - {}",
// //                             selected_resource.name, selected_resource.instructions
// //                         );
// //                         break Ok(()); // Exit after selection
// //                     }
// //                 }
// //                 crossterm::event::KeyCode::Esc => {
// //                     break Ok(()); // Exit on Escape
// //                 }
// //                 _ => (),
// //                 // crossterm::event::KeyCode::Backspace => todo!(),
// //                 // crossterm::event::KeyCode::Left => todo!(),
// //                 // crossterm::event::KeyCode::Right => todo!(),
// //                 // crossterm::event::KeyCode::Home => todo!(),
// //                 // crossterm::event::KeyCode::End => todo!(),
// //                 // crossterm::event::KeyCode::PageUp => todo!(),
// //                 // crossterm::event::KeyCode::PageDown => todo!(),
// //                 // crossterm::event::KeyCode::Tab => todo!(),
// //                 // crossterm::event::KeyCode::BackTab => todo!(),
// //                 // crossterm::event::KeyCode::Delete => todo!(),
// //                 // crossterm::event::KeyCode::Insert => todo!(),
// //                 // crossterm::event::KeyCode::F(_) => todo!(),
// //                 // crossterm::event::KeyCode::Char(_) => todo!(),
// //                 // crossterm::event::KeyCode::Null => todo!(),
// //                 // crossterm::event::KeyCode::CapsLock => todo!(),
// //                 // crossterm::event::KeyCode::ScrollLock => todo!(),
// //                 // crossterm::event::KeyCode::NumLock => todo!(),
// //                 // crossterm::event::KeyCode::PrintScreen => todo!(),
// //                 // crossterm::event::KeyCode::Pause => todo!(),
// //                 // crossterm::event::KeyCode::Menu => todo!(),
// //                 // crossterm::event::KeyCode::KeypadBegin => todo!(),
// //                 // crossterm::event::KeyCode::Media(_media_key_code) => todo!(),
// //                 // crossterm::event::KeyCode::Modifier(_modifier_key_code) => todo!(),
// //             }
// //         }
// //     }
// // }

// // // #[tokio::main]
// // // async fn main() {
// // //     if let Err(e) = run().await {
// // //         eprintln!("Error: {}", e);
// // //     }
// // // }

// use color_eyre::Result;
// use crossterm::event::KeyEventKind;
// use ratatui::{
//     crossterm::event::{self, Event, KeyCode, KeyModifiers},
//     layout::{Constraint, Layout, Rect},
//     style::{self, Color, Modifier, Style},
//     text::Text,
//     widgets::{Block, BorderType, Cell, Row, Table, TableState},
//     DefaultTerminal, Frame,
// };
// use style::palette::tailwind;

// const ITEM_HEIGHT: usize = 4; // Height of each row in the table

// // Define some color themes from tailwind
// const PALETTES: [tailwind::Palette; 4] = [
//     tailwind::BLUE,
//     tailwind::EMERALD,
//     tailwind::INDIGO,
//     tailwind::RED,
// ];

// // Placeholder data for Azure resource types
// #[derive(Debug)]
// struct AzureResource {
//     name: String,
//     description: String,
//     instructions: String,
// }

// impl AzureResource {
//     const fn new(name: &str, description: &str, instructions: &str) -> Self {
//         AzureResource {
//             name: name.to_string(),
//             description: description.to_string(),
//             instructions: instructions.to_string(),
//         }
//     }

//     // Returns the fields as an array for easy iteration in the table rows
//     fn as_ref_array(&self) -> [&String; 3] {
//         [&self.name, &self.description, &self.instructions]
//     }
// }

// // Application state to hold the table data and configuration
// struct App {
//     state: TableState,
//     resources: Vec<AzureResource>,
//     longest_item_lens: (u16, u16, u16), // lengths of name, description, instructions columns
//     colors: TableColors,
//     color_index: usize,
// }

// impl App {
//     fn new() -> Self {
//         let resources = vec![
//             AzureResource::new(
//                 "keyvault",
//                 "Azure Key Vault for secure secrets",
//                 "Configure secret storage",
//             ),
//             AzureResource::new(
//                 "storageaccount",
//                 "Azure Storage Account for blob storage",
//                 "Set up storage containers",
//             ),
//             AzureResource::new(
//                 "appservice",
//                 "Azure App Service for hosting web apps",
//                 "Deploy web applications",
//             ),
//             AzureResource::new(
//                 "cosmosdb",
//                 "Azure Cosmos DB for globally distributed databases",
//                 "Create a new database",
//             ),
//             AzureResource::new(
//                 "vnet",
//                 "Azure Virtual Network for network isolation",
//                 "Set up virtual networks",
//             ),
//             AzureResource::new(
//                 "functionapp",
//                 "Azure Function App for serverless computing",
//                 "Deploy serverless functions",
//             ),
//         ];

//         let longest_item_lens = (20, 25, 30); // Placeholder for column widths
//         Self {
//             state: TableState::default().with_selected(0),
//             resources,
//             longest_item_lens,
//             colors: TableColors::new(&PALETTES[0]),
//             color_index: 0,
//         }
//     }

//     fn next_row(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.resources.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }

//     fn previous_row(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.resources.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }

//     pub fn run(mut self, terminal: DefaultTerminal) -> Result<()> {
//         loop {
//             terminal.draw(|frame| self.draw(frame))?;

//             if let Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press {
//                     let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
//                     match key.code {
//                         KeyCode::Esc => return Ok(()),
//                         KeyCode::Char('j') | KeyCode::Down => self.next_row(),
//                         KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//     }

//     fn draw(&mut self, frame: &mut Frame) {
//         let layout = Layout::default().constraints([Constraint::Min(5)].as_ref());
//         let rects = layout.split(frame.area());

//         self.render_table(frame, rects[0]);
//     }

//     fn render_table(&mut self, frame: &mut Frame, area: Rect) {
//         let header = ["Resource Type", "Description", "Instructions"]
//             .into_iter()
//             .map(Cell::from)
//             .collect::<Row>()
//             .style(Style::default().fg(Color::Yellow))
//             .height(1);

//         let rows = self.resources.iter().enumerate().map(|(i, resource)| {
//             let color = if i % 2 == 0 {
//                 Color::DarkGray
//             } else {
//                 Color::LightGreen
//             };
//             let item = resource.as_ref_array();
//             item.into_iter()
//                 .map(|content| Cell::from(Text::from(content)))
//                 .collect::<Row>()
//                 .style(Style::new().fg(Color::White).bg(color))
//                 .height(ITEM_HEIGHT as u16)
//         });

//         let table = Table::new(rows, self.state.clone())
//             .header(header)
//             .widths(&[
//                 Constraint::Length(self.longest_item_lens.0 + 1),
//                 Constraint::Length(self.longest_item_lens.1 + 1),
//                 Constraint::Min(self.longest_item_lens.2),
//             ])
//             .block(
//                 Block::default()
//                     .borders(ratatui::widgets::Borders::ALL)
//                     .title("Azure Resources"),
//             );

//         frame.render_stateful_widget(table, area, &mut self.state);
//     }
// }

// // Colors for the table UI
// struct TableColors {
//     buffer_bg: Color,
//     header_bg: Color,
//     header_fg: Color,
// }

// impl TableColors {
//     const fn new(color: &tailwind::Palette) -> Self {
//         Self {
//             buffer_bg: tailwind::SLATE.c950,
//             header_bg: color.c900,
//             header_fg: tailwind::SLATE.c200,
//         }
//     }
// }

// fn main() -> Result<()> {
//     let terminal = ratatui::init();
//     let app_result = App::new().run(terminal);
//     ratatui::restore();
//     app_result
// }
