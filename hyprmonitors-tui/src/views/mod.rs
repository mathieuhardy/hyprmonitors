pub mod advanced_settings;
pub mod help;
pub mod main;
pub mod mirror_picker;
pub mod profile_input;
pub mod profile_picker;
pub mod resolution_picker;
pub mod scale_picker;
pub mod workspace_assign;

use crossterm::event::*;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::colors::*;
use crate::state::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActiveView {
    AdvancedSettings,
    Help,
    Main,
    MirrorPicker,
    ProfileInput,
    ProfilePicker,
    ResolutionPicker,
    ScalePicker,
    WorkspaceAssign,
}

pub trait View {
    async fn reset(&mut self, state: &State);
    fn render(&self, frame: &mut Frame, state: &mut State);
    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error>;
}

fn render_popup_list(
    f: &mut Frame,
    title: &str,
    items: Vec<ListItem>,
    cursor: usize,
    w_pct: u16,
    h_pct: u16,
) {
    let area = centered_rect(w_pct, h_pct, f.area());
    f.render_widget(Clear, area);

    let mut state = ListState::default();
    state.select(Some(cursor));

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(C_LIGHT_GREY)),
        )
        .highlight_style(Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(C_FG));

    f.render_stateful_widget(list, area, &mut state);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
