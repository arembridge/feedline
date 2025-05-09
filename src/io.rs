use crate::colors::{GREEN, RED, RESET, YELLOW};
use crate::verbosity::Verbosity;
use std::fs::{OpenOptions, metadata};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};

pub fn is_non_empty_file(path: &str) -> bool {
    match metadata(path) {
        Ok(meta) => {
            let ft = meta.file_type();
            ft.is_file() && !ft.is_symlink() && meta.len() > 0
        }
        Err(_) => false,
    }
}

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

pub fn process_files<I: Iterator<Item = String>>(files: I, verbosity: Verbosity) -> bool {
    let mut has_error = false;

    for file in files {
        if !is_non_empty_file(&file) {
            if verbosity != Verbosity::QUIET {
                eprintln!(
                    "{YELLOW}skipped (empty, symlink or not a file):{RESET} {}",
                    file
                );
            }
            continue;
        }

        match add_feedline(&file) {
            Ok(true) => {
                if verbosity != Verbosity::QUIET {
                    println!("{GREEN}updated:{RESET} {}", file);
                }
            }
            Ok(false) => {
                if verbosity != Verbosity::QUIET {
                    println!("{YELLOW}skipped (feedline already present):{RESET} {}", file);
                }
            }
            Err(e) => {
                eprintln!("{RED}error processing {}: {}{RESET}", file, e);
                has_error = true;
            }
        }
    }

    has_error
}

pub fn collect_files(args: &[String]) -> Box<dyn Iterator<Item = String>> {
    if args.is_empty() {
        Box::new(BufReader::new(io::stdin()).lines().filter_map(Result::ok))
    } else {
        Box::new(args.to_owned().into_iter())
    }
}
