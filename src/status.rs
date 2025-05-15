use std::cmp::Ordering;

use colored::{ColoredString, Colorize};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq)]
pub enum STATUS {
    SUCCESS,
    WARN,
    SKIP,
    ERROR,
}

impl STATUS {
    pub fn get_colored_string(&self) -> ColoredString {
        use STATUS::*;
        match self {
            SUCCESS => "SUCCESS".green(),
            SKIP => "SKIP".yellow(),
            WARN => "WARN".red(),
            ERROR => "ERROR".red(),
        }
    }
}

impl Ord for STATUS {
    fn cmp(&self, other: &Self) -> Ordering {
        use STATUS::*;
        let rank = |s| match s {
            SUCCESS => 0,
            SKIP => 1,
            WARN => 2,
            ERROR => 3,
        };
        rank(*self).cmp(&rank(*other))
    }
}
