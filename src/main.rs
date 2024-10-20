mod cli;
mod gui;

use log::*;

fn main() {
    stderrlog::new()
        .verbosity(log::LevelFilter::Info)
        .module(module_path!())
        .init()
        .unwrap();

    info!(
        "Conduct {} ({})",
        env!("GIT_DESCRIPTION"),
        env!("GIT_BRANCH"),
    );

    let result = cli::cli();

    match result {
        cli::CliResult::ShowUI => gui::gui(),
        _ => return,
    }
}
