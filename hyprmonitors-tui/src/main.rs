mod actions;
mod colors;
mod hyprland;
mod mouse;
mod options;
mod snap_mode;
mod state;
mod status;
mod ui;
mod views;

use std::time::Duration;

use clap::Parser;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use actions::*;
use common::error::*;
use state::*;

use options::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _opts = Options::parse();
    let mut stdout = std::io::stdout();

    // Setup terminal
    enable_raw_mode()?;

    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let size = terminal.size()?;
    let mut state = State::new(size.width, size.height).await?;

    // Loop until quit is requested
    let result = run_state(&mut terminal, &mut state).await;

    // Reset settings
    disable_raw_mode()?;

    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    terminal.show_cursor()?;

    result
}

async fn run_state(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    state: &mut State,
) -> Result<(), Error> {
    let mut ui = ui::UserInterface::new();

    loop {
        // Render the UI
        terminal.draw(|frame| ui.render(frame, state))?;

        // Poll events
        if crossterm::event::poll(Duration::from_millis(50))? {
            let event = crossterm::event::read()?;

            match ui.handle_event(state, event).await? {
                Action::Quit => {
                    break;
                }

                Action::ChangeView(view) => {
                    ui.set_active_view(view);
                    ui.reset(state);
                }

                _ => {}
            }
        }
    }

    Ok(())
}
