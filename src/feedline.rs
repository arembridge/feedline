use crate::status::STATUS;
use crate::feedline_result::FeedlineResult;
use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

fn ensure_feedline(filepath: &str) -> Result<FeedlineResult, std::io::Error> {
    let mut file = OpenOptions::new().read(true).write(true).open(filepath)?;

    // Check if the file is empty
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        return Ok(FeedlineResult {
            file: filepath.to_string(),
            status: STATUS::SKIP,
            message: Some("file is empty".to_string()),
        });
    }

    file.seek(SeekFrom::End(-1))?;

    let mut last_byte = [0u8; 1];
    file.read_exact(&mut last_byte)?;

    if last_byte != [b'\n'] {
        file.write_all(b"\n")?;
        return Ok(FeedlineResult {
            file: filepath.to_string(),
            status: STATUS::SUCCESS,
            message: None,
        });
    }

    Ok(FeedlineResult {
        file: filepath.to_string(),
        status: STATUS::SKIP,
        message: Some("file already has a feedline".to_string()),
    })
}

fn fix_file(filepath: String) -> FeedlineResult {
    let path = Path::new(&filepath);

    if !path.exists() {
        return FeedlineResult {
            file: filepath,
            status: STATUS::ERROR,
            message: Some("path does not exist".to_string()),
        };
    }

    if path.is_dir() {
        return FeedlineResult {
            file: filepath,
            status: STATUS::SKIP,
            message: Some("path is a directory".to_string()),
        };
    }

    if path.is_symlink() {
        return FeedlineResult {
            file: filepath,
            status: STATUS::WARN,
            message: Some("path is a symlink".to_string()),
        };
    }

    if !path.is_file() {
        return FeedlineResult {
            file: filepath,
            status: STATUS::ERROR,
            message: Some("path is not a file".to_string()),
        };
    }

    match ensure_feedline(filepath.as_str()) {
        Ok(result) => result,
        Err(_) => FeedlineResult {
            file: filepath,
            status: STATUS::ERROR,
            message: Some("failed checking feedline".to_string()),
        },
    }
}

pub fn fix_files(files: Vec<String>) -> Vec<FeedlineResult> {
    let mut results = Vec::new();
    for file in files {
        let result = fix_file(file);
        results.push(result);
    }
    results
}
