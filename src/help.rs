use crate::colors::{BOLD, RESET};

pub fn print_help() {
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
