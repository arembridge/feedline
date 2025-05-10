use crate::args::{ColorOption, CommandArgs};
use crate::colors::{GREEN, RED, RESET, YELLOW};
use crate::feedline::{FeedlineResult, STATUS};
use crate::verbosity::Verbosity;

fn get_status_rank(status: STATUS) -> i32 {
    match status {
        STATUS::SUCCESS => 0,
        STATUS::SKIP => 1,
        STATUS::WARN => 2,
        STATUS::ERROR => 3,
    }
}

fn get_status_color(status: STATUS) -> String {
    match status {
        STATUS::SUCCESS => GREEN.to_string(),
        STATUS::SKIP => YELLOW.to_string(),
        STATUS::WARN => YELLOW.to_string(),
        STATUS::ERROR => RED.to_string(),
    }
}

pub fn print_results(command_args: &CommandArgs, mut results: Vec<FeedlineResult>) {
    // Do not report on "SUCCESS" if verbosity is quiet
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

    // Do not report on "WARN" or "SKIP" if verbosity is not verbose
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

    // Sort results by status (if enabled)
    if command_args.sort {
        results.sort_by(|a, b| {
            let status_cmp = get_status_rank(a.status).cmp(&get_status_rank(b.status));
            if status_cmp == std::cmp::Ordering::Equal {
                a.file.cmp(&b.file)
            } else {
                status_cmp
            }
        });
    }

    for result in results {
        let terminal_color = get_status_color(result.status);
        let status = if command_args.color == ColorOption::ALWAYS {
            format!("{}{:?}{}", terminal_color, result.status, RESET)
        } else {
            format!("{:?}", result.status)
        };

        match result.message {
            Some(m) => println!("{} {} ({})", status, result.file, m),
            _ => println!("{} {}", status, result.file),
        }
    }
}
