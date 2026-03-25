use crossterm::event::*;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;
use common::hyprland::*;
use common::profile::*;

use crate::actions::*;
use crate::colors::*;
use crate::state::*;
use crate::status::*;

use super::*;

pub const ADV_FIELD_BITDEPTH: usize = 0;
pub const ADV_FIELD_COLORMODE: usize = 1;
pub const ADV_FIELD_SDR_BRIGHTNESS: usize = 2;
pub const ADV_FIELD_SDR_SATURATION: usize = 3;
pub const ADV_FIELD_VRR: usize = 4;
pub const ADV_FIELD_TRANSFORM: usize = 5;
pub const ADV_FIELD_COUNT: usize = 6;

pub struct AdvanceSettingsView {
    field: usize,
}

impl AdvanceSettingsView {
    pub fn new() -> Self {
        Self { field: 0 }
    }
}

impl View for AdvanceSettingsView {
    fn reset(&mut self, _state: &State) {
        self.field = 0
    }

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let area = centered_rect(50, 60, frame.area());
        frame.render_widget(Clear, area);

        let monitor = match state.monitor() {
            Some(m) => m,
            None => return,
        };

        let field = self.field;
        let is_hdr = monitor.advanced.color_mode == ColorMode::Hdr
            || monitor.advanced.color_mode == ColorMode::HdrEdid;

        let mut lines: Vec<Line> = vec![
            Line::from(Span::styled(
                " Advanced Display Settings",
                Style::default().fg(C_ORANGE).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        let fields = [
            (
                ADV_FIELD_BITDEPTH,
                "Bit Depth",
                format!("{}-bit", monitor.advanced.bit_depth),
            ),
            (
                ADV_FIELD_COLORMODE,
                "Color Mode",
                monitor.advanced.color_mode.menu_entry().to_string(),
            ),
            (
                ADV_FIELD_SDR_BRIGHTNESS,
                "SDR Brightness",
                format!("{:.2}", monitor.advanced.sdr_brightness),
            ),
            (
                ADV_FIELD_SDR_SATURATION,
                "SDR Saturation",
                format!("{:.2}", monitor.advanced.sdr_saturation),
            ),
            (
                ADV_FIELD_VRR,
                "VRR",
                monitor.advanced.vrr.menu_entry().to_string(),
            ),
            (
                ADV_FIELD_TRANSFORM,
                "Transform",
                monitor.advanced.transform.menu_entry().to_string(),
            ),
        ];

        for &(idx, label, ref value) in &fields {
            // Skip SDR fields if not HDR
            if !is_hdr && (idx == ADV_FIELD_SDR_BRIGHTNESS || idx == ADV_FIELD_SDR_SATURATION) {
                continue;
            }
            let selected = idx == field;
            let prefix = if selected { "▶ " } else { "  " };
            let style = if selected {
                Style::default().fg(C_BLUE).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(C_FG)
            };
            lines.push(Line::from(Span::styled(
                format!("{}{:<18} {}", prefix, label, value),
                style,
            )));
        }

        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            " ←/→ adjust  Space toggle  Enter confirm  Esc cancel",
            Style::default().fg(C_GREY),
        )));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(C_LIGHT_GREY))
            .title(" Advanced Settings ");

        frame.render_widget(Paragraph::new(lines).block(block), area);
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        let mon = match state.monitor_mut() {
            Some(m) => m,
            None => {
                return Ok(Action::ChangeView(ActiveView::Main));
            }
        };

        let is_hdr = mon.advanced.color_mode == ColorMode::Hdr
            || mon.advanced.color_mode == ColorMode::HdrEdid;
        let field = self.field;

        match key.code {
            KeyCode::Esc => {
                state.set_status(Status::Cancelled("Advanced settings".to_string()));
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Enter => {
                state.set_status(Status::AdvancedSettingsApplied);
                action = Action::ChangeView(ActiveView::Main);
            }

            KeyCode::Up => {
                // Navigate up, skip SDR fields if not HDR
                self.field = prev_adv_field(field, is_hdr);
            }

            KeyCode::Down => {
                self.field = next_adv_field(field, is_hdr);
            }

            KeyCode::Left => adjust_adv_field(mon, field, -1),

            KeyCode::Right => adjust_adv_field(mon, field, 1),

            KeyCode::Char(' ') => toggle_adv_field(mon, field),

            _ => {}
        }

        Ok(action)
    }
}

fn next_adv_field(field: usize, is_hdr: bool) -> usize {
    let mut next = (field + 1) % ADV_FIELD_COUNT;
    if !is_hdr && (next == ADV_FIELD_SDR_BRIGHTNESS || next == ADV_FIELD_SDR_SATURATION) {
        next = ADV_FIELD_VRR;
    }
    next
}

fn prev_adv_field(field: usize, is_hdr: bool) -> usize {
    let mut prev = if field == 0 {
        ADV_FIELD_COUNT - 1
    } else {
        field - 1
    };
    if !is_hdr && (prev == ADV_FIELD_SDR_BRIGHTNESS || prev == ADV_FIELD_SDR_SATURATION) {
        prev = ADV_FIELD_COLORMODE;
    }
    prev
}

fn adjust_adv_field(mon: &mut Monitor, field: usize, delta: i32) {
    match field {
        ADV_FIELD_BITDEPTH => {
            mon.advanced.bit_depth = if mon.advanced.bit_depth == 8 { 10 } else { 8 };
        }

        ADV_FIELD_COLORMODE => {
            mon.advanced.color_mode = if delta < 0 {
                mon.advanced.color_mode.previous()
            } else {
                mon.advanced.color_mode.next()
            };
        }

        ADV_FIELD_SDR_BRIGHTNESS => {
            mon.advanced.sdr_brightness =
                (mon.advanced.sdr_brightness + delta as f64 * 0.1).clamp(0.5, 2.0);
            mon.advanced.sdr_brightness = (mon.advanced.sdr_brightness * 10.0).round() / 10.0;
        }

        ADV_FIELD_SDR_SATURATION => {
            mon.advanced.sdr_saturation =
                (mon.advanced.sdr_saturation + delta as f64 * 0.1).clamp(0.5, 1.5);
            mon.advanced.sdr_saturation = (mon.advanced.sdr_saturation * 10.0).round() / 10.0;
        }

        ADV_FIELD_VRR => {
            mon.advanced.vrr = if delta < 0 {
                mon.advanced.vrr.previous()
            } else {
                mon.advanced.vrr.next()
            };
        }

        ADV_FIELD_TRANSFORM => {
            mon.advanced.transform = if delta < 0 {
                mon.advanced.transform.previous()
            } else {
                mon.advanced.transform.next()
            };
        }

        _ => {}
    }
}

fn toggle_adv_field(mon: &mut Monitor, field: usize) {
    match field {
        ADV_FIELD_BITDEPTH => {
            mon.advanced.bit_depth = if mon.advanced.bit_depth == 8 { 10 } else { 8 };
        }

        ADV_FIELD_VRR => {
            mon.advanced.vrr = mon.advanced.vrr.next();
        }

        _ => {}
    }
}
