mod cli;
mod gui;

use log::*;

fn main() {
    stderrlog::new().verbosity(5).init().unwrap();

    info!(
        "Conduct {}{} ({})",
        env!("GIT_DESCRIPTION"),
        env!("GIT_SUFFIX"),
        env!("GIT_BRANCH"),
    );

    if std::env::args().len() > 1 {
        cli::cli();
    } else {
        gui::gui()
    }
}
