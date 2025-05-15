use crate::args::ColorOption;
use crate::verbosity::Verbosity;
use colored::{ColoredString, Colorize};
use std::io::{self, IsTerminal};

#[derive(Debug, Clone)]
pub struct Printer {
    pub use_color: bool,
    pub verbosity_level: Verbosity,
}

impl Printer {
    pub fn new(color_option: &ColorOption, verbosity_level: Verbosity) -> Self {
        let use_color = match color_option {
            ColorOption::ALWAYS => true,
            ColorOption::NEVER => false,
            ColorOption::AUTO => io::stdout().is_terminal(),
        };
        let verbosity_level = match verbosity_level {
            Verbosity::VERBOSE => Verbosity::VERBOSE,
            Verbosity::NORMAL => Verbosity::NORMAL,
            Verbosity::QUIET => Verbosity::QUIET,
        };
        Self {
            use_color,
            verbosity_level,
        }
    }

    pub fn print(&self, message_parts: Vec<ColoredString>, message_max_verbosity: Verbosity) {
        if message_max_verbosity > self.verbosity_level {
            return;
        }

        if self.use_color {
            let joined_string = message_parts
                .into_iter()
                .map(|item| item.to_string())
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("{}", joined_string);
        } else {
            let joined_string = message_parts
                .into_iter()
                .map(|item| item.normal().clear().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            eprintln!("{}", joined_string);
        }
    }
}
