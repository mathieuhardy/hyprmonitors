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

pub struct ProfileInputView {
    input: String,
}

impl ProfileInputView {
    pub fn new() -> Self {
        Self {
            input: String::new(),
        }
    }
}

impl View for ProfileInputView {
    fn reset(&mut self, _state: &State) {
        self.input = String::new();
    }

    fn render(&self, frame: &mut Frame, _state: &mut State) {
        let area = centered_rect(40, 20, frame.area());
        frame.render_widget(Clear, area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Save Profile ")
            .border_style(Style::default().fg(C_LIGHT_GREY));

        let text = vec![
            Line::from(""),
            Line::from(Span::styled(" Profile name:", Style::default().fg(C_BLUE))),
            Line::from(Span::styled(
                format!(" {}█", self.input),
                Style::default().fg(C_FG).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                " Enter: save  Esc: cancel",
                Style::default().fg(C_GREY),
            )),
        ];

        frame.render_widget(Paragraph::new(text).block(block), area);
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Profile save".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Enter => {
                state.save_to_profile(self.input.trim().to_string()).await?;
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Backspace => {
                self.input.pop();
            }

            KeyCode::Char(c) => {
                self.input.push(c);
            }

            _ => {}
        }

        Ok(action)
    }
}
