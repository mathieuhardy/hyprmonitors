use crossterm::event::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub const SCALE_OPTIONS: &[f64] = &[0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];

pub struct ScalePickerView {
    cursor_position: usize,
}

impl ScalePickerView {
    pub fn new() -> Self {
        Self { cursor_position: 0 }
    }
}

impl View for ScalePickerView {
    fn reset(&mut self, state: &State) {
        let scale = state.monitor_scale();

        self.cursor_position = SCALE_OPTIONS
            .iter()
            .position(|&s| (s - scale).abs() < 0.01)
            .unwrap_or(2);
    }

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let items: Vec<ListItem> = SCALE_OPTIONS
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let marker = if i == self.cursor_position {
                    "▶ "
                } else {
                    "  "
                };
                let cur_scale = state.monitor_scale();
                let active_marker = if (s - cur_scale).abs() < 0.01 {
                    " ✓"
                } else {
                    ""
                };
                ListItem::new(format!("{}{:.2}x{}", marker, s, active_marker))
            })
            .collect();

        render_popup_list(
            frame,
            " Scale Selection ",
            items,
            self.cursor_position,
            30,
            20,
        );
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Scale selection".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }

            KeyCode::Down => {
                if self.cursor_position + 1 < SCALE_OPTIONS.len() {
                    self.cursor_position += 1;
                }
            }

            KeyCode::Enter => {
                let scale = SCALE_OPTIONS[self.cursor_position];
                state.apply_scale(scale);
                action = Action::ChangeView(ActiveView::Main);
            }

            _ => {}
        }

        Ok(action)
    }
}
