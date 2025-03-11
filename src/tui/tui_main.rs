// use std::io;

// use crossterm::terminal;
// use ratatui::{prelude::CrosstermBackend, Terminal};

// use super::app::{App, InfraConfig};

// pub fn tui_main_f() -> io::Result<InfraConfig> {
//     let backend = CrosstermBackend::new(io::stdout());
//     let mut terminal = Terminal::new(backend)?;

//     terminal::enable_raw_mode()?;
//     terminal.clear()?;

//     let mut app = App::new();
//     let _app_result = app.run(&mut terminal)?;

//     // terminal.show_cursor()?;

//     // Check if confirmation is false, meaning validation failed or the user didn't confirm
//     if !app.confirmation {
//         ratatui::restore();
//         return Err(io::Error::new(
//             io::ErrorKind::Other,
//             "Action was not confirmed",
//         ));
//     }

//     terminal.clear()?;
//     app.show_exit_screen = true;
//     ratatui::restore();

//     Ok(app.config)
// }

use std::io::{self, Write}; // Import Write for flushing

use crossterm::terminal;
use ratatui::{prelude::CrosstermBackend, Terminal};

use super::app::{App, ProjectConfig};

pub fn tui_main() -> io::Result<ProjectConfig> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    terminal::enable_raw_mode()?;
    terminal.hide_cursor()?;
    terminal.clear()?;

    io::stdout().flush()?;

    let mut app = App::new();
    let _app_result = app.run(&mut terminal)?;

    terminal.flush()?; // Ensure immediate output after drawing

    if !app.confirmation {
        ratatui::restore();
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Action was not confirmed",
        ));
    }

    terminal.clear()?;
    app.show_exit_screen = true;
    terminal.show_cursor()?;
    ratatui::restore();

    terminal.flush()?; // Flush again before exit

    Ok(app.config)
}
