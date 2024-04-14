use std::io::{self, stdout, Stdout};

use crossterm::{cursor::SetCursorStyle, execute, terminal::*};
use ratatui::prelude::*;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen, SetCursorStyle::SteadyBar,)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(
        stdout(),
        SetCursorStyle::DefaultUserShape,
        LeaveAlternateScreen
    )?;
    disable_raw_mode()?;
    Ok(())
}
