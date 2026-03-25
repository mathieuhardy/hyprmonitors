use crate::views::*;

pub enum Action {
    ChangeView(ActiveView),
    None,
    Quit,
}
