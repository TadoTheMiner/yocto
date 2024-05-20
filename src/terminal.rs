use std::io::{stderr, Result};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use yocto_editor::Terminal;

pub fn initialize() -> Result<Terminal> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    Terminal::new(CrosstermBackend::new(std::io::stderr()))
}
pub fn clean_up() -> Result<()> {
    // shutdown down: reset terminal back to original state
    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
