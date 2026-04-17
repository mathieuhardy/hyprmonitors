use crossterm::event::*;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;

use crate::actions::*;
use crate::colors::*;
use crate::state::*;
use crate::views::*;

pub struct HelpView {
    pub scroll: u16,
}

impl HelpView {
    pub fn new() -> Self {
        Self { scroll: 0 }
    }
}

impl View for HelpView {
    async fn reset(&mut self, _state: &State) {
        self.scroll = 0
    }

    fn render(&self, frame: &mut Frame, _state: &mut State) {
        let area = frame.area();
        frame.render_widget(Clear, area);

        let all_lines: Vec<Line> = vec![
            Line::from(Span::styled(
                " HyprMonitors — Keyboard & Mouse Reference",
                Style::default().fg(C_ORANGE).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                " Navigation",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  ←↑↓→               Move selected monitor by grid step"),
            Line::from("  Shift+←↑↓→         Move by 10× grid step"),
            Line::from("  Tab / Shift+Tab    Cycle through monitors"),
            Line::from(""),
            Line::from(Span::styled(
                " Display",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  G                  Cycle grid size (1,8,16,32,64 px)"),
            Line::from("  S                  Cycle snap mode (Off/Edges/Centers/Both)"),
            Line::from("  Enter              Toggle monitor active/inactive"),
            Line::from("  L                  Open scale picker"),
            Line::from("  R                  Open resolution/refresh rate picker"),
            Line::from("  M                  Open mirror configuration"),
            Line::from("  A                  Open advanced display settings"),
            Line::from("  P                  Open profile picker"),
            Line::from("  Ctrl+↑↓            Decrease/increase scale by 0.05"),
            Line::from(""),
            Line::from(Span::styled(
                " Workspaces",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  W                  Open workspace assignment dialog"),
            Line::from("                     Assign integer workspace IDs to monitors"),
            Line::from("                     Prints hyprctl workspace commands on apply"),
            Line::from(""),
            Line::from(Span::styled(
                " Actions",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  F1                 Apply configuration to Hyprland"),
            Line::from("  F2                 Save to current configuration"),
            Line::from("  F3                 Save to a profile"),
            Line::from("  Ctrl+Z             Revert to previous state (prints revert commands)"),
            Line::from(""),
            Line::from(Span::styled(
                " Mouse",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  Left click         Select monitor"),
            Line::from("  Left drag          Move monitor (with snap)"),
            Line::from("  Right click        Toggle active"),
            Line::from("  Scroll wheel       Adjust scale"),
            Line::from(""),
            Line::from(Span::styled(
                " Other",
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD),
            )),
            Line::from("  ?                  Toggle this help"),
            Line::from("  Q / Esc / Ctrl+C   Quit"),
            Line::from(""),
            Line::from(Span::styled(
                " Help navigation: ↑/↓ scroll  Esc/q close",
                Style::default().fg(C_LIGHT_GREY),
            )),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Help ")
            .border_style(Style::default().fg(C_GREY));

        frame.render_widget(
            Paragraph::new(all_lines)
                .block(block)
                .scroll((self.scroll, 0)),
            area,
        );
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        match key.code {
            KeyCode::Esc | KeyCode::Char('?') => {
                self.reset(state).await;
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                self.scroll = self.scroll.saturating_sub(1);
            }

            KeyCode::Down => {
                self.scroll += 1;
            }

            KeyCode::PageUp => {
                self.scroll = self.scroll.saturating_sub(10);
            }

            KeyCode::PageDown => {
                self.scroll += 10;
            }

            KeyCode::Home => {
                self.scroll = 0;
            }

            KeyCode::End => {
                self.scroll = 999;
            }

            _ => {}
        }

        Ok(action)
    }
}
