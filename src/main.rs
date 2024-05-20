use std::panic;
mod app;
mod terminal;
use app::App;
use clap::{crate_authors, crate_description, crate_version, Arg, ArgMatches, Command};
use color_eyre::{config::HookBuilder, eyre, Result};
use terminal::{clean_up, initialize};
fn main() -> Result<()> {
    install_hooks()?;
    App::new(initialize()?, cli().get_one::<String>("FILE"))?.run()?;
    clean_up()?;
    Ok(())
}

fn cli() -> ArgMatches {
    Command::new("yocto")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::new("FILE").help("File to edit"))
        .get_matches()
}

/// This replaces the standard color_eyre panic and error hooks with hooks that
/// restore the terminal before printing the panic or error.
pub fn install_hooks() -> Result<()> {
    let (panic_hook, eyre_hook) = HookBuilder::default().into_hooks();

    // convert from a color_eyre PanicHook to a standard panic hook
    let panic_hook = panic_hook.into_panic_hook();
    panic::set_hook(Box::new(move |panic_info| {
        terminal::clean_up().unwrap();
        panic_hook(panic_info);
    }));

    // convert from a color_eyre EyreHook to a eyre ErrorHook
    let eyre_hook = eyre_hook.into_eyre_hook();
    eyre::set_hook(Box::new(
        move |error: &(dyn std::error::Error + 'static)| {
            terminal::clean_up().unwrap();
            eyre_hook(error)
        },
    ))?;

    Ok(())
}
