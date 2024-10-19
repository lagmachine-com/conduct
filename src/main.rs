mod cli;
mod gui;

fn main() {
    stderrlog::new().verbosity(5).init().unwrap();

    if std::env::args().len() > 1 {
        cli::cli();
    } else {
        gui::gui()
    }
}
