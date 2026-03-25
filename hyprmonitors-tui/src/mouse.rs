use crossterm::event::*;

use crate::state::*;

pub fn handle_mouse(state: &mut State, mouse: MouseEvent) {
    let x = mouse.column as i32;
    let y = mouse.row as i32;

    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(idx) = state.hit_test(x, y) {
                // A monitor was clicked
                state.set_selected(idx);
                state.begin_drag_monitor(x, y);
            }
        }

        MouseEventKind::Drag(MouseButton::Left) => {
            state.drag_move_monitor(x, y);
        }

        MouseEventKind::Up(MouseButton::Left) => {
            state.end_drag_monitor();
        }

        MouseEventKind::ScrollUp => {
            state.adjust_scale(0.05);
        }

        MouseEventKind::ScrollDown => {
            state.adjust_scale(-0.05);
        }

        _ => {}
    }
}
