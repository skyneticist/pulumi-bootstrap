use anyhow::Result;
use cli::Cli;

mod commands {
    pub mod config;
    pub mod project;
    pub mod snippet;
}

mod helpers {
    pub mod config_helper;
    pub mod entrypoint_helper;
    pub mod pipeline_helper;
}

mod cli;
mod tui {
    pub mod app;
    mod constants;
    mod enums;
    pub mod snippet_select;
    pub mod tui_main;
    mod ui;
    pub mod validation;
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();
    cli.run().await;
    Ok(())
}
