use std::{
    io::{stderr, Stderr},
    panic,
};

use clap::{crate_authors, crate_description, crate_name, crate_version, Arg, ArgMatches, Command};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::prelude::{CrosstermBackend, Terminal};

use yocto_editor::{app::App, Result};
fn main() -> Result<()> {
    init_panic_handler();
    let result = run_app();
    clean_up()?;
    result
}

fn run_app() -> Result<()> {
    App::new(initialize_terminal()?, cli().get_one::<String>("FILE"))?.run()
}

fn cli() -> ArgMatches {
    Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new("FILE").help("File to edit"))
        .get_matches()
}

pub fn init_panic_handler() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        clean_up().unwrap();
        original_hook(panic_info)
    }));
}

fn initialize_terminal() -> Result<Terminal<CrosstermBackend<Stderr>>> {
    // startup: Enable raw mode for the terminal, giving us fine control over user input
    enable_raw_mode()?;
    execute!(stderr(), EnterAlternateScreen)?;

    // Initialize the terminal backend using crossterm
    Ok(Terminal::new(CrosstermBackend::new(std::io::stderr()))?)
}

fn clean_up() -> Result<()> {
    // shutdown down: reset terminal back to original state
    execute!(stderr(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
