use crossterm::event::*;
use ratatui::layout::*;
use ratatui::style::*;
use ratatui::text::*;
use ratatui::widgets::*;
use ratatui::*;

use common::error::*;
use common::hyprland::*;

use crate::actions::*;
use crate::colors::*;
use crate::state::*;
use crate::status::*;
use crate::views::*;

pub struct MainView {}

impl MainView {
    pub fn new() -> Self {
        Self {}
    }
}

impl View for MainView {
    async fn reset(&mut self, _state: &State) {}

    fn render(&self, frame: &mut Frame, state: &mut State) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // header
                Constraint::Min(8),    // desktop canvas
                Constraint::Length(3), // monitor details
                Constraint::Length(5), // footer / keys
            ])
            .split(area);

        render_header(frame, state, chunks[0]);
        render_desktop(frame, state, chunks[1]);
        render_details(frame, state, chunks[2]);
        render_footer(frame, state, chunks[3]);
    }

    async fn handle_key(&mut self, key: KeyEvent, state: &mut State) -> Result<Action, Error> {
        let mut action = Action::None;

        let step = if key.modifiers.contains(KeyModifiers::SHIFT) {
            state.grid_px() * 10
        } else {
            state.grid_px()
        };

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                action = Action::Quit;
            }

            // Monitor move
            KeyCode::Left => state.move_monitor(-step, 0),
            KeyCode::Right => state.move_monitor(step, 0),
            KeyCode::Up if key.modifiers.is_empty() => state.move_monitor(0, -step),
            KeyCode::Down if key.modifiers.is_empty() => state.move_monitor(0, step),

            // Monitor selection
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    state.select_previous_monitor();
                } else {
                    state.select_next_monitor();
                }
            }

            // Toggle active
            KeyCode::Enter => state.toggle_selected_monitor(),

            // Grid
            KeyCode::Char('g') | KeyCode::Char('G') => {
                state.cycle_grid();
            }

            // Snap
            KeyCode::Char('s') | KeyCode::Char('S') => state.cycle_snap(),

            // Scale fine-tune
            KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.adjust_scale(-0.05)
            }

            KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.adjust_scale(0.05)
            }

            // Sub-views
            KeyCode::Char('l') | KeyCode::Char('L') => {
                action = Action::ChangeView(ActiveView::ScalePicker);
            }

            KeyCode::Char('r') | KeyCode::Char('R') => {
                action = Action::ChangeView(ActiveView::ResolutionPicker);
            }

            KeyCode::Char('m') | KeyCode::Char('M') => {
                action = Action::ChangeView(ActiveView::MirrorPicker);
            }

            KeyCode::Char('a') | KeyCode::Char('A') => {
                action = Action::ChangeView(ActiveView::AdvancedSettings);
            }

            KeyCode::Char('w') | KeyCode::Char('W') => {
                action = Action::ChangeView(ActiveView::WorkspaceAssign);
            }

            KeyCode::Char('p') | KeyCode::Char('P') => {
                action = Action::ChangeView(ActiveView::ProfilePicker);
            }

            // Actions
            KeyCode::F(1) => {
                state.save_rollback();

                state.apply_configuration().await?;
                state.set_status(Status::ConfigurationApplied);
            }

            KeyCode::F(2) => {
                let path = state.save_configuration().await?;
                state.set_status(Status::ConfigurationSaved(path));
            }

            KeyCode::F(3) => {
                action = Action::ChangeView(ActiveView::ProfileInput);
            }

            KeyCode::Char('z') | KeyCode::Char('Z')
                if key.modifiers.contains(KeyModifiers::CONTROL) =>
            {
                state.revert();
            }

            // Help
            KeyCode::Char('?') => {
                action = Action::ChangeView(ActiveView::Help);
            }

            _ => {}
        }

        Ok(action)
    }
}

fn render_header(frame: &mut Frame, state: &State, area: Rect) {
    let title = Span::styled(
        format!(
            " HyprMon  │  Monitors: {}  │  Grid: {}px  │  Snap: {}",
            state.monitors_count(),
            state.grid_px(),
            state.snap_mode()
        ),
        Style::default().fg(C_ORANGE).add_modifier(Modifier::BOLD),
    );

    frame.render_widget(Paragraph::new(Line::from(title)), area);
}

fn render_desktop(frame: &mut Frame, state: &State, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(C_GREY))
        .title(" Layout ");
    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Draw guides
    for guide in state.guides() {
        match guide.kind {
            GuideKind::Vertical => {
                let (tx, _) = state.world_to_term(guide.value, 0);
                let tx = (inner.x as i32 + tx) as u16;
                if tx >= inner.x && tx < inner.x + inner.width {
                    for row in inner.y..inner.y + inner.height {
                        let rect = Rect::new(tx, row, 1, 1);
                        frame.render_widget(
                            Paragraph::new("│").style(Style::default().fg(C_RED)),
                            rect,
                        );
                    }
                }
            }
            GuideKind::Horizontal => {
                let (_, ty) = state.world_to_term(0, guide.value);
                let ty = (inner.y as i32 + ty) as u16;
                if ty >= inner.y && ty < inner.y + inner.height {
                    let rect = Rect::new(inner.x, ty, inner.width, 1);
                    frame.render_widget(
                        Paragraph::new("─".repeat(inner.width as usize))
                            .style(Style::default().fg(C_RED)),
                        rect,
                    );
                }
            }
        }
    }

    // Draw monitors
    let selected_monitor_name = state.monitor().map(|m| m.name.clone()).unwrap_or_default();
    for mon in state.monitors() {
        let is_selected = mon.name == selected_monitor_name;

        let (tx, ty) = state.world_to_term(mon.position.x, mon.position.y);
        let (tw, th) = {
            let (x2, y2) = state.world_to_term(
                mon.position.x + mon.effective_width(),
                mon.position.y + mon.effective_height(),
            );
            ((x2 - tx).max(4), (y2 - ty).max(2))
        };

        let abs_x = inner.x as i32 + tx;
        let abs_y = inner.y as i32 + ty;

        if abs_x < inner.x as i32
            || abs_y < inner.y as i32
            || abs_x + tw >= (inner.x + inner.width) as i32
            || abs_y + th >= (inner.y + inner.height) as i32
        {
            continue; // out of bounds
        }

        let rect = Rect::new(abs_x as u16, abs_y as u16, tw as u16, th as u16);

        let (border_color, label_color) = if is_selected {
            (C_PURPLE, C_FG)
        } else if mon.active {
            (C_BLUE, C_FG)
        } else {
            (C_GREY, C_FG)
        };

        // Build label
        let mut label_lines = vec![];

        // Monitor name + resolution
        let name_line = format!(" {} ", mon.name);
        label_lines.push(name_line);

        let res_line = format!(
            " {}x{}@{:.0}Hz ",
            mon.resolution.width, mon.resolution.height, mon.resolution.hz
        );
        label_lines.push(res_line);

        let scale_line = format!(" {:.2}x ", mon.scale);
        label_lines.push(scale_line);

        // Workspace line
        if !mon.workspaces.is_empty() {
            let ws_str: Vec<String> = mon.workspaces.iter().map(|id| id.to_string()).collect();
            label_lines.push(format!(" ws:[{}] ", ws_str.join(",")));
        }

        // Status badges
        let mut badges = vec![];
        if mon.advanced.bit_depth == 10 {
            badges.push("10bit".to_string());
        }
        if mon.advanced.color_mode != ColorMode::Unknown
            && mon.advanced.color_mode != ColorMode::Srgb
            && mon.advanced.color_mode != ColorMode::Auto
        {
            badges.push(mon.advanced.color_mode.to_string());
        }
        if mon.advanced.vrr != Vrr::Off {
            badges.push("VRR".to_string());
        }
        if mon.advanced.transform != MonitorTransform::Normal {
            badges.push("ROT".to_string());
        }
        if mon.is_mirrored {
            badges.push(format!("MIRROR: {}", mon.mirror_source));
        }
        if !badges.is_empty() {
            label_lines.push(format!(" {} ", badges.join(" ")));
        }

        // Inactive label
        if !mon.active {
            label_lines.insert(0, " [DISABLED] ".into());
        }

        let content = label_lines.join("\n");
        let style = Style::default().fg(label_color);
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .style(
                #[allow(clippy::if_same_then_else)]
                if is_selected {
                    Style::default().bg(C_BG)
                } else if mon.active {
                    Style::default()
                } else {
                    Style::default()
                },
            );

        frame.render_widget(Clear, rect);
        frame.render_widget(
            Paragraph::new(content)
                .block(block)
                .style(style)
                .wrap(Wrap { trim: false }),
            rect,
        );
    }
}

fn render_details(frame: &mut Frame, state: &State, area: Rect) {
    let text = if let Some(m) = state.monitor() {
        let ws_str = if m.workspaces.is_empty() {
            "none".into()
        } else {
            m.workspaces
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!(
            " {} │ {}x{}@{:.2}Hz │ pos: ({},{}) │ scale: {:.2} │ ws: {} │ {}",
            m.name,
            m.resolution.width,
            m.resolution.height,
            m.resolution.hz,
            m.position.x,
            m.position.y,
            m.scale,
            ws_str,
            if m.active { "active" } else { "DISABLED" }
        )
    } else {
        " No monitor selected".into()
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Selected Monitor ")
        .border_style(Style::default().fg(C_GREY));

    frame.render_widget(
        Paragraph::new(text)
            .block(block)
            .style(Style::default().fg(C_FG)),
        area,
    );
}

fn render_footer(frame: &mut Frame, state: &State, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(1)])
        .split(area);

    // Status message
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Status ")
        .border_style(Style::default().fg(C_GREY));

    frame.render_widget(
        Paragraph::new(state.status().to_string().as_str())
            .block(block)
            .style(Style::default().fg(C_YELLOW).add_modifier(Modifier::BOLD)),
        chunks[0],
    );

    // Command helper
    let keys = " F1 Apply │ F2 Save │ F3 Save as | P Profiles │ G Grid │ S Snap │ L Scale │ R Resolution │ M Mirror │ A Adv │ W Workspaces │ C+Z Revert │ ? Help │ Q Quit";

    frame.render_widget(
        Paragraph::new(keys).style(Style::default().fg(C_LIGHT_GREY)),
        chunks[1],
    );
}
