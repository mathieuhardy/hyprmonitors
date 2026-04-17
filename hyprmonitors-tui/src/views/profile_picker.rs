use crossterm::event::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;
use common::profile::*;

use crate::actions::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub struct ProfilePickerView {
    cursor_position: usize,
    profiles: Vec<Profile>,
}

impl ProfilePickerView {
    pub fn new() -> Self {
        Self {
            cursor_position: 0,
            profiles: Vec::new(),
        }
    }
}

impl View for ProfilePickerView {
    async fn reset(&mut self, _state: &State) {
        self.cursor_position = 0;
        self.profiles = load_profiles().await.unwrap_or_default();
    }

    fn render(&self, frame: &mut Frame, _state: &mut State) {
        let profiles_names: Vec<_> = self
            .profiles
            .iter()
            .filter_map(|profile| get_profile_name(profile).into())
            .collect();

        let mut items = Vec::new();

        for (idx, name) in profiles_names.iter().enumerate() {
            let marker = if idx == self.cursor_position {
                "▶ "
            } else {
                "  "
            };

            items.push(ListItem::new(format!("{}{}", marker, name)));
        }

        render_popup_list(frame, " Profiles ", items, self.cursor_position, 40, 15);
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Profile selection".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }

            KeyCode::Down => {
                let total = self.profiles.len();

                if self.cursor_position + 1 < total {
                    self.cursor_position += 1;
                }
            }

            KeyCode::Enter => {
                if let Some(profile) = self.profiles.get(self.cursor_position) {
                    state.set_profile(profile.clone());

                    state.set_status(Status::ProfileLoaded(get_profile_name(profile)));
                    action = Action::ChangeView(ActiveView::Main);
                }
            }

            _ => {}
        }

        Ok(action)
    }
}

fn get_profile_name(profile: &Profile) -> String {
    profile
        .filepath
        .as_ref()
        .and_then(|p| p.file_stem().map(|name| name.to_string_lossy().to_string()))
        .unwrap_or_default()
}
