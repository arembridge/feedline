use crate::verbosity::Verbosity;
use clap::{Parser, ValueEnum};
use colored::{ColoredString, Colorize};
use std::io::{self, BufRead, IsTerminal};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum ColorOption {
    #[value(help = "Always use color in output")]
    ALWAYS,

    #[value(help = "Never use color in output")]
    NEVER,

    #[value(help = "Automatically detect if color should be used")]
    AUTO,
}

impl ColorOption {
    pub fn get_colored_message(&self) -> ColoredString {
        match self {
            ColorOption::ALWAYS => "ALWAYS".green().bold(),
            ColorOption::NEVER => "NEVER".normal(),
            ColorOption::AUTO => "AUTO".blue().bold(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Make sure there is an empty line at the end of the files provided",
    long_about = "FEEDLINE:\n\nA command-line utility that makes sure there is an empty line at the end of the files provided.",
    after_help = "\x1b[1m\x1b[4mExamples:\x1b[0m\n\x1b[1m# Format files explicitly in verbose mode (with color)\x1b[0m\n> feedline -v --color=always file1.txt\n\n\x1b[1m# Pipe the files in a folder (using bash expansion) to feedline\x1b[0m\n> ls examples/*.txt | feedline --sort\n\n\x1b[1m# Find files, pipe them to feedline, then filter with grep\x1b[0m\n> find ./src/ -type f | feedline --color=never | grep '^SKIP.*'"
)]
pub struct CLIArgs {
    #[arg(
        long,
        ignore_case = true,
        default_value = "auto",
        help = "Control when to use colored output (always or never)"
    )]
    pub color: ColorOption,

    #[arg(
        short,
        action = clap::ArgAction::Count,
        help = "Increase output verbosity. Use multiple times for more detail (-v, -vv, -vvv)."
    )]
    pub verbose: u8,

    #[arg(
        short,
        long,
        action = clap::ArgAction::SetTrue,
        help = "Silence all output that is not an error (overrides any `-v` flags)"  // TODO: add more info beyond -v
    )]
    pub quiet: bool,

    #[arg(
        short,
        long,
        default_value = "false",
        value_parser = parse_bool,
        help = "Sort by status (ERROR > WARN > SKIP > SUCCESS), then alphabetically."
    )]
    pub sort: bool,

    /// Files to process. If none provided, reads from stdin.
    #[arg(
        value_name = "FILES",
        help = "Files to process (if no files provided, read from standard input)"
    )]
    pub files: Vec<String>,
}

pub struct CommandArgs {
    pub files: Vec<String>,
    pub color: ColorOption,
    pub sort: bool,
    pub verbosity: Verbosity,
}

fn parse_bool(src: &str) -> Result<bool, String> {
    match src.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(format!("invalid boolean value: {}", src)),
    }
}

fn process_stdin() -> Vec<String> {
    let mut files = vec![];
    let stdin = io::stdin();
    for (idx, line) in stdin.lock().lines().enumerate() {
        if let Ok(file) = line {
            if !file.trim().is_empty() {
                files.push(file);
            }
        } else if let Err(file) = line {
            eprintln!("Failed to process line: {}::{}", file, idx);
        }
    }
    files
}

pub fn parse_args() -> CommandArgs {
    let args = CLIArgs::parse();
    let files: Vec<String>;

    let verbosity = if args.quiet {
        Verbosity::QUIET
    } else if args.verbose > 0 {
        Verbosity::VERBOSE
    } else {
        Verbosity::NORMAL
    };

    if args.files.is_empty() && !io::stdin().is_terminal() {
        // No files provided, process stdin
        files = process_stdin();
    } else {
        // Process files
        files = args.files;
    }

    let color = args.color;
    let sort = args.sort;

    CommandArgs {
        files,
        color,
        sort,
        verbosity,
    }
}
