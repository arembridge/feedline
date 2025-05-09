mod args;
mod colors;
mod feedline;
mod print;
mod verbosity;

use crate::feedline::fix_files;
use crate::print::print_results;
use args::parse_args;

fn main() {
    let command_args = parse_args();

    let results = fix_files(command_args.files.clone());

    print_results(&command_args, results);
}
