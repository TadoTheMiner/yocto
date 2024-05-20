use std::panic;
mod app;
mod terminal;
use app::App;

use clap::{crate_authors, crate_description, crate_version, Arg, ArgMatches, Command};
use terminal::clean_up;
use yocto_editor::Result;
fn main() -> Result<()> {
    init_panic_handler();
    let result = run_app();
    clean_up()?;
    result
}

fn run_app() -> Result<()> {
    App::new(terminal::initialize()?, cli().get_one::<String>("FILE"))?.run()
}

fn cli() -> ArgMatches {
    Command::new("yocto")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new("FILE").help("File to edit"))
        .get_matches()
}

fn init_panic_handler() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        clean_up().unwrap();
        original_hook(panic_info)
    }));
}
