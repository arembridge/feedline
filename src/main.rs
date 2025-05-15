mod args;
mod feedline;
mod feedline_result;
mod print;
mod status;
mod verbosity;

use std::io::{self, IsTerminal};

use colored::Colorize;

use verbosity::Verbosity;

use crate::feedline::fix_files;
use crate::status::STATUS;
use args::parse_args;

fn main() {
    let command_args = parse_args();
    let printer = print::Printer::new(&command_args.color, command_args.verbosity.clone());

    printer.eprint(
        vec![
            vec!["files: ".blue()],
            command_args
                .files
                .clone() // Fix clone
                .into_iter()
                .map(|item| item.normal())
                .collect()
        ]
        .concat(),
        Verbosity::VERBOSE,
    );
    printer.eprint(
        vec!["color: ".blue(), command_args.color.get_colored_message()],
        Verbosity::VERBOSE,
    );
    printer.eprint(
        vec![
            "verbosity: ".blue(),
            command_args.verbosity.get_colored_message(),
        ],
        Verbosity::VERBOSE,
    );
    printer.eprint(
        vec![
            "sort: ".blue(),
            match command_args.sort {
                true => "true".green(),
                false => "false".red(),
            },
        ],
        Verbosity::VERBOSE,
    );

    let mut results = fix_files(command_args.files.clone());
    if command_args.sort {
        results.sort();
    }

    for result in results {
        let verbosity_min = match result.status {
            STATUS::SUCCESS => Verbosity::NORMAL,
            STATUS::WARN => Verbosity::VERBOSE,
            STATUS::SKIP => Verbosity::NORMAL,
            STATUS::ERROR => Verbosity::QUIET,
        };
        if verbosity_min > printer.verbosity_level {
            continue;
        };
        let message_parts = result.get_message_parts();
        printer.eprint(message_parts, verbosity_min);
    }

    if !io::stdout().is_terminal() {
        for file in command_args.files {
            printer.print(vec![file.into()], Verbosity::QUIET);
        }
    }
}
