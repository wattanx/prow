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
use events::Action;
use std::time::Duration;

use crate::types::AppMode;

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

    let mut terminal = ratatui::init();
    let mut state = app::AppState::new(config);

    loop {
        terminal.draw(|frame| ui::render(frame, &state))?;

        let Some(action) = events::poll_event(Duration::from_millis(250))? else {
            continue;
        };

        match state.mode {
            AppMode::Filter => match action {
                Action::MoveDown => {
                    let max = state.all_repos.len();
                    if state.filter_cursor_index < max {
                        state.filter_cursor_index += 1;
                    }
                }
                Action::MoveUp => {
                    if state.filter_cursor_index > 0 {
                        state.filter_cursor_index -= 1;
                    }
                }
                Action::FilterToggle => {
                    if state.filter_cursor_index == 0 {
                        state.select_all_repos();
                    } else {
                        state.toggle_repo(state.filter_cursor_index - 1);
                    }
                }
                Action::OpenInBrowser => {
                    config::save_filtered_repos(
                        &state.selected_repos.iter().cloned().collect::<Vec<_>>(),
                    )?;
                    state.mode = AppMode::List;
                    state.selected_index = 0;
                }
                Action::FilterCancel => {
                    state.mode = AppMode::List;
                }
                _ => {}
            },
            AppMode::List => match action {
                Action::Quit => state.should_quit = true,
                Action::MoveDown => {
                    let max = state.current_prs().len().saturating_sub(1);
                    if state.selected_index < max {
                        state.selected_index += 1;
                    }
                }
                Action::MoveUp => {
                    if state.selected_index > 0 {
                        state.selected_index -= 1;
                    }
                }
                Action::OpenInBrowser => {
                    let prs = state.current_prs();
                    if let Some(pr) = prs.get(state.selected_index) {
                        let _ = browser::open_in_browser(&pr.url);
                    }
                }
                Action::NextSection => state.switch_section(1),
                Action::PrevSection => state.switch_section(-1),
                Action::GoToTop => state.selected_index = 0,
                Action::GoToBottom => {
                    let max = state.current_prs().len().saturating_sub(1);
                    state.selected_index = max;
                }
                Action::ToggleSort => {
                    state.sort_order = state.sort_order.toggle();
                }
                Action::EnterFilter => {
                    if !state.all_repos.is_empty() {
                        state.mode = AppMode::Filter;
                        state.filter_cursor_index = 0;
                    }
                }
                Action::Refresh => {
                    // TODO: Phase 4
                }
                _ => {}
            },
        }

        if state.should_quit {
            break;
        }
    }
    ratatui::restore();

    Ok(())
}
