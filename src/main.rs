use std::env;
use std::fs::{OpenOptions, metadata};
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

// ANSI color codes
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";

// Verbosity levels
#[derive(PartialEq)]
enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

// Configuration structure for parsing args
struct Config {
    verbosity: Verbosity,
    files: Vec<String>,
}

fn main() {
    let config = parse_args();
    let file_list = collect_files(&config.files);
    let has_errors = process_files(file_list, config.verbosity);

    if has_errors {
        std::process::exit(1);
    }
}

fn parse_args() -> Config {
    let mut verbosity = Verbosity::Normal;
    let mut files = vec![];

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-q" | "--quiet" => verbosity = Verbosity::Quiet,
            "-v" | "--verbose" => verbosity = Verbosity::Verbose,
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => files.push(arg),
        }
    }

    Config { verbosity, files }
}

fn collect_files(args: &[String]) -> Box<dyn Iterator<Item = String>> {
    if args.is_empty() {
        Box::new(io::stdin().lock().lines().filter_map(Result::ok))
    } else {
        Box::new(args.to_owned().into_iter())
    }
}

fn process_files<I: Iterator<Item = String>>(files: I, verbosity: Verbosity) -> bool {
    let mut has_error = false;

    for file in files {
        // Verbose mode: show received files
        if verbosity == Verbosity::Verbose {
            println!("{BLUE}received:{RESET} {}", file);
        }

        // Skip non-regular files, symlinks, and empty files
        if !is_non_empty_file(&file) {
            if verbosity == Verbosity::Verbose {
                eprintln!(
                    "{DIM}{YELLOW}skipped (non-regular, symlink, or empty):{RESET} {}",
                    file
                );
            }
            continue;
        }

        // Try to add a newline
        match add_feedline(&file) {
            Ok(true) => {
                // Normal mode and above print updated files
                if verbosity != Verbosity::Quiet {
                    println!("{GREEN}updated:{RESET} {}", file);
                }
            }
            Ok(false) => {
                // Verbose prints skipped files, Normal and Quiet don't
                if verbosity == Verbosity::Verbose {
                    println!("{YELLOW}skipped:{RESET} {}", file);
                }
            }
            Err(e) => {
                // Print errors in all verbosity levels
                eprintln!("{RED}error processing {}: {}{RESET}", file, e);
                has_error = true;
            }
        }
    }

    has_error
}

fn is_non_empty_file(path: &str) -> bool {
    match metadata(path) {
        Ok(meta) => {
            let ft = meta.file_type();
            ft.is_file() && !ft.is_symlink() && meta.len() > 0
        }
        Err(_) => false,
    }
}

fn add_feedline(path: &str) -> std::io::Result<bool> {
    let mut file = OpenOptions::new().read(true).append(true).open(path)?;
    file.seek(SeekFrom::End(-1))?;
    let mut last_byte = [0u8; 1];
    file.read_exact(&mut last_byte)?;
    if last_byte != [b'\n'] {
        file.write_all(b"\n")?;
        return Ok(true);
    }
    Ok(false)
}

fn print_help() {
    println!(
        "\
{BOLD}feedline{RESET} – Ensures files end with a single newline.

{BOLD}USAGE:{RESET}
    feedline [OPTIONS] [FILES...]

    You can also pipe in a list of files:
        git diff --name-only | feedline

{BOLD}OPTIONS:{RESET}
    -v, --verbose     Show detailed output (received/skipped/updated)
    -q, --quiet       Only show errors
    -h, --help        Show this help message and exit

Default mode prints only files that were updated and errors.

{BOLD}BEHAVIOR:{RESET}
    • Only processes regular, non-symlink, non-empty files.
    • Adds a newline to the end of a file if it's missing.
    • Exits with non-zero status if any file fails to process.
"
    );
}
