use crate::colors::{GREEN, RED, RESET, YELLOW};
use crate::verbosity::Verbosity;
use std::fs::{OpenOptions, metadata};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};

// Check if the file is non-empty and not a symlink
pub fn is_non_empty_file(path: &str) -> bool {
    match metadata(path) {
        Ok(meta) => {
            let ft = meta.file_type();
            ft.is_file() && !ft.is_symlink() && meta.len() > 0
        }
        Err(_) => false,
    }
}

// Add newline at the end of the file if necessary
pub fn add_feedline(path: &str) -> std::io::Result<bool> {
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

// Process all files based on verbosity level
pub fn process_files<I: Iterator<Item = String>>(files: I, verbosity: Verbosity) -> bool {
    let mut has_error = false;

    for file in files {
        // Skip non-regular files, symlinks, and empty files
        if !is_non_empty_file(&file) {
            if verbosity == Verbosity::Verbose {
                eprintln!(
                    "{YELLOW}skipped (non-regular, symlink, or empty):{RESET} {}",
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

// Collect files, either from stdin or from args
pub fn collect_files(args: &[String]) -> Box<dyn Iterator<Item = String>> {
    if args.is_empty() {
        // Wrap stdin in a BufReader, which gives us access to lines()
        Box::new(BufReader::new(io::stdin()).lines().filter_map(Result::ok))
    } else {
        Box::new(args.to_owned().into_iter())
    }
}
