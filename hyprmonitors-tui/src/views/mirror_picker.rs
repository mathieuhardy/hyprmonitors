use crossterm::event::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub struct MirrorPickerView {
    cursor_position: usize,
}

impl MirrorPickerView {
    pub fn new() -> Self {
        Self { cursor_position: 0 }
    }
}

impl View for MirrorPickerView {
    fn reset(&mut self, _state: &State) {
        self.cursor_position = 0;
    }

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let mirrorable_monitors = state.mirrorable_monitors();

        let first_entry = if self.cursor_position == 0 {
            "▶ [None - disable mirror]"
        } else {
            "  [None - disable mirror]"
        };

        let mut items = vec![ListItem::new(first_entry)];

        for (i, monitor) in mirrorable_monitors.iter().enumerate() {
            let idx = i + 1;

            let marker = if idx == self.cursor_position {
                "▶ "
            } else {
                "  "
            };

            items.push(ListItem::new(format!("{}{}", marker, monitor.name)));
        }

        render_popup_list(
            frame,
            " Mirror Source ",
            items,
            self.cursor_position,
            40,
            15,
        );
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Mirror selection".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }

            KeyCode::Down => {
                let mirrorable_monitors = state.mirrorable_monitors();
                let total = mirrorable_monitors.len() + 1; // +1 for "none"

                if self.cursor_position + 1 < total {
                    self.cursor_position += 1;
                }
            }

            KeyCode::Enter => {
                let mirrorable_monitors = state.mirrorable_monitors();

                let monitor_name = if self.cursor_position == 0 {
                    None
                } else {
                    Some(mirrorable_monitors[self.cursor_position - 1].name.clone())
                };

                state.mirror_picker_select_current(monitor_name);

                action = Action::ChangeView(ActiveView::Main);
            }

            _ => {}
        }

        Ok(action)
    }
}
