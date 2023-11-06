#![forbid(unsafe_code)]

use crate::helpers::{create_marker, match_source_pattern};
use regex::Regex;
use std::{
    fs,
    io::{stdout, Write},
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MoveError {
    #[error("Not able to replace existing file: {0:?}")]
    UnforcedReplace(PathBuf),
    #[error("Destination directory does not exist.")]
    InvalidDirectory,
    #[error("Files for pattern {0:?} not found.")]
    NonexistentPattern(String),
}

/// Checks if teplate mathes a filename.
/// If so, tries to move source file to destination file parsed from the template and prints changed files.
///
/// # Arguments
///
/// * `source_path` - a path to the source file
/// * `destination_path` - a path to the destination template
/// * `expression` - a template
/// * `force_flag` - allow replacing files
pub fn try_to_move(
    source_path: &Path,
    destination_path: &Path,
    expression: &str,
    force_flag: bool,
) -> Result<bool, MoveError> {
    let source_file = source_path.file_name().unwrap().to_str().unwrap();
    let destination_template = Path::new(destination_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    if !match_source_pattern(expression, source_file) {
        return Ok(false);
    }

    let markers = Regex::new(expression)
        .unwrap()
        .captures(source_file)
        .unwrap();
    let mut destination_file = destination_template.to_string();
    for i in 1..markers.len() {
        destination_file = destination_file.replace(&create_marker(i), &markers[i]);
    }

    let source_directory = source_path.parent().unwrap();
    let destination_directory = Path::new(destination_path).parent().unwrap();

    let destination = destination_directory.join(destination_file);
    let source = source_directory.join(source_file);

    if destination.as_path().exists() && !force_flag {
        return Err(MoveError::UnforcedReplace(destination));
    }
    let _ = writeln!(stdout(), "{:?} -> {:?}", source, destination);

    match fs::rename(source, destination) {
        Ok(()) => Ok(true),
        Err(_err) => Err(MoveError::InvalidDirectory),
    }
}
