use colored::{ColoredString, Colorize};

// Verbosity levels
#[derive(PartialEq, Eq, PartialOrd, Debug, Clone)]
pub enum Verbosity {
    QUIET,
    NORMAL,
    VERBOSE,
}

impl Verbosity {
    pub fn get_colored_message(&self) -> ColoredString {
        match self {
            Verbosity::QUIET => "QUIET".blue(),
            Verbosity::NORMAL => "NORMAL".green(),
            Verbosity::VERBOSE => "VERBOSE".red(),
        }
    }
}
