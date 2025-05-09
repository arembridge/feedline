mod colors;
mod config;
mod help;
mod io;
mod verbosity;

use crate::config::parse_args;
use crate::io::{collect_files, process_files};

fn main() {
    let config = parse_args();
    let file_list = collect_files(&config.files);
    let has_errors = process_files(file_list, config.verbosity);

    if has_errors {
        std::process::exit(1);
    }
}
