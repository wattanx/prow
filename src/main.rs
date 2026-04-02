mod app;
mod browser;
mod config;
mod events;
mod github;
mod pr_classifier;
mod types;
mod ui;
mod updater;

use anyhow::Result;
use clap::{Parser, Subcommand};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(name = "prow", version = VERSION, about = "GitHub PR management TUI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Update to latest version
    Upgrade,
    /// Uninstall prow
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Upgrade) => {
            updater::self_update().await?;
            return Ok(());
        }
        Some(Command::Uninstall) => {
            updater::self_uninstall().await?;
            return Ok(());
        }
        None => {}
    }

    let config = config::load_config()?;

    // TODO (Phase 2): Initialize terminal, create AppState, run event loop
    // let mut terminal = ratatui::init();
    // let mut state = app::AppState::new(config);
    // loop {
    //     terminal.draw(|frame| ui::render(frame, &state))?;
    //     // handle events...
    //     if state.should_quit { break; }
    // }
    // ratatui::restore();

    println!("prow v{VERSION} — TUI not yet implemented");

    Ok(())
}
