use crossterm::event::*;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::colors::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub struct WorkspaceAssignView {
    input: String,
}

impl WorkspaceAssignView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }

    pub fn apply_to_monitor(&self, state: &mut State) {
        let input = self.input.trim().to_string();

        let workspaces_ids: Vec<usize> = input
            .split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .collect();

        state.apply_workspaces_to_monitor(workspaces_ids);
    }
}

impl View for WorkspaceAssignView {
    async fn reset(&mut self, state: &State) {
        self.input = state.workspaces_as_str();
    }

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let area = centered_rect(60, 70, frame.area());
        frame.render_widget(Clear, area);

        let inner_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(5), Constraint::Length(4)])
            .split(area);

        let selected_monitor_name = state.monitor().map(|m| m.name.clone()).unwrap_or_default();
        let mut selected_index = 0;

        // Monitor list with workspace assignments
        let items: Vec<ListItem> = state
            .monitors()
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let is_selected = m.name == selected_monitor_name;
                if is_selected {
                    selected_index = i;
                }

                let ws_str = if m.workspaces.is_empty() {
                    "—".into()
                } else {
                    m.workspaces
                        .iter()
                        .map(|id| id.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                let marker = if is_selected { "▶ " } else { "  " };
                let style = if is_selected {
                    Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(C_FG)
                };
                ListItem::new(Span::styled(
                    format!("{}{:<20} ws: {}", marker, m.name, ws_str),
                    style,
                ))
            })
            .collect();

        let mut list_state = ListState::default();
        list_state.select(Some(selected_index));

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Workspace Assignments  (Tab=select monitor) ")
            .border_style(Style::default().fg(C_LIGHT_GREY));

        frame.render_stateful_widget(
            List::new(items).block(block),
            inner_chunks[0],
            &mut list_state,
        );

        // Input area
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Enter workspace IDs (comma-separated, e.g. 1,2,3) ")
            .border_style(Style::default().fg(C_LIGHT_GREY));

        let input_text = format!("{}█", self.input);
        frame.render_widget(
            Paragraph::new(input_text)
                .block(input_block)
                .style(Style::default().fg(C_FG)),
            inner_chunks[1],
        );
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Workspace assignment".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Enter => {
                self.apply_to_monitor(state);
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Down => {
                self.apply_to_monitor(state);
                state.select_next_monitor();
                self.input = state.workspaces_as_str();
            }

            KeyCode::Up => {
                self.apply_to_monitor(state);
                state.select_previous_monitor();
                self.input = state.workspaces_as_str();
            }

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) if c.is_ascii_digit() || c == ',' || c == ' ' || c == '-' => {
                self.input.push(c);
            }

            _ => {}
        }

        Ok(action)
    }
}
