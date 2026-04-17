use crossterm::event::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub struct ResolutionPickerView {
    cursor_position: usize,
}

impl ResolutionPickerView {
    pub fn new() -> Self {
        Self { cursor_position: 0 }
    }
}

impl View for ResolutionPickerView {
    async fn reset(&mut self, _state: &State) {
        self.cursor_position = 0;
    }

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let resolutions = state.monitor_resolutions();

        let items: Vec<ListItem> = resolutions
            .iter()
            .enumerate()
            .map(|(i, resolution)| {
                let marker = if i == self.cursor_position {
                    "▶ "
                } else {
                    "  "
                };

                ListItem::new(format!("{}{}", marker, resolution))
            })
            .collect();

        render_popup_list(
            frame,
            " Resolution / Refresh Rate ",
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
                state.set_status(Status::Cancelled("Resolution selection".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }

            KeyCode::Down => {
                let count = state.monitor_resolutions().len();

                if self.cursor_position + 1 < count {
                    self.cursor_position += 1;
                }
            }

            KeyCode::Enter => {
                let resolutions = state.monitor_resolutions();

                if let Some(resolution) = resolutions.get(self.cursor_position) {
                    state.set_monitor_resolution(resolution);
                    state.set_status(Status::Resolution(*resolution));
                }

                action = Action::ChangeView(ActiveView::Main);
            }

            _ => {}
        }

        Ok(action)
    }
}
