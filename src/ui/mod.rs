pub mod pr_list;
pub mod pr_row;
pub mod repo_filter;
pub mod section_list;
pub mod status_bar;

use ratatui::Frame;

use crate::app::AppState;

/// Main render function — dispatches to widgets based on app state.
///
/// Layout (top to bottom):
///   1. SectionList (section tabs with counts)
///   2. PRList or RepoFilter (depending on mode)
///   3. StatusBar (keybindings, sort order, last updated)
///
/// See: src/app.tsx — return JSX (lines 157-178)
pub fn render(frame: &mut Frame, state: &AppState) {
    todo!("Split frame into 3 vertical chunks and render each widget")
}
