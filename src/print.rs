use crate::args::{ColorOption, CommandArgs};
use crate::colors::{GREEN, RED, RESET, YELLOW};
use crate::feedline::{FeedlineResult, STATUS};
use crate::verbosity::Verbosity;

fn get_status_color(status: STATUS) -> String {
    match status {
        STATUS::SUCCESS => GREEN.to_string(),
        STATUS::SKIP => YELLOW.to_string(),
        STATUS::WARN => YELLOW.to_string(),
        STATUS::ERROR => RED.to_string(),
    }
}

pub fn print_results(command_args: &CommandArgs, mut results: Vec<FeedlineResult>) {
    // Do not report on SUCCESS/SKIP/WARN if verbosity is quiet
    if command_args.verbosity == Verbosity::QUIET {
        results = results
            .into_iter()
            .filter(|result| {
                result.status != STATUS::SUCCESS
                    && result.status != STATUS::SKIP
                    && result.status != STATUS::WARN
            })
            .collect();
    }

    // Print CommandArgs if verbosity is verbose
    if command_args.verbosity == Verbosity::VERBOSE {
        let verbosity_string = if command_args.color == ColorOption::ALWAYS {
            format!("{YELLOW}verbosity:{RESET} {:?}", command_args.verbosity)
        } else {
            format!("verbosity: {:?}", command_args.verbosity)
        };
        println!("{}", verbosity_string);

        let files_string = if command_args.color == ColorOption::ALWAYS {
            format!("{YELLOW}files:{RESET}")
        } else {
            format!("files:")
        };
        println!("{}", files_string);
        for file in &command_args.files {
            println!("\t{}", file);
        }
    }

    // Sort results by status, then alphabetical (if enabled)
    if command_args.sort {
        results.sort_by(|a, b| a.cmp(b));
    }

    for result in results {
        let terminal_color = get_status_color(result.status);
        let status = if command_args.color == ColorOption::ALWAYS {
            format!("{}{:?}{}", terminal_color, result.status, RESET)
        } else {
            format!("{:?}", result.status)
        };

        match result.message {
            Some(m) => {
                let message = if command_args.color == ColorOption::ALWAYS {
                    format!("{}({}){}", terminal_color, m, RESET)
                } else {
                    format!("({:?})", m)
                };
                println!("{} {} {}", status, result.file, message)
            }
            _ => println!("{} {}", status, result.file),
        }
    }
}
