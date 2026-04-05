pub mod pr_list;
pub mod pr_row;
pub mod repo_filter;
pub mod section_list;
pub mod status_bar;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};

use crate::app::AppState;
use crate::app::empty_message;
use crate::types::AppMode;

/// Main render function — dispatches to widgets based on app state.
///
/// Layout (top to bottom):
///   1. SectionList (section tabs with counts)
///   2. PRList or RepoFilter (depending on mode)
///   3. StatusBar (keybindings, sort order, last updated)
///
/// See: src/app.tsx — return JSX (lines 157-178)
pub fn render(frame: &mut Frame, state: &AppState) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let counts = state.section_counts();
    section_list::render(frame, chunks[0], state.active_section, &counts);

    match state.mode {
        AppMode::Filter => {
            repo_filter::render(
                frame,
                chunks[1],
                &state.all_repos,
                &state.selected_repos,
                state.filter_cursor_index,
            );
        }
        AppMode::List => {
            let prs = state.current_prs();
            pr_list::render(
                frame,
                chunks[1],
                &prs,
                state.selected_index,
                empty_message(state.active_section),
                state.loading,
            );
        }
    }

    status_bar::render(
        frame,
        chunks[2],
        state.mode,
        state.sort_order,
        state.last_updated,
        state.loading,
    );
}
