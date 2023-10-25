//! > **Command line utility for mass moving or renaming files according to a template**
//!
//! Moves files that match template from \<SOURCE> directory to \<DESTINATION> directory and renames
//! it according to the template. Both arguments shoud contain existing directory path.
//! It is forbidden to use ``^, $, +, -, ?, (, ), [, ], {, }, |`` in templates.
//!
//! \<SOURCE> contains a template with symbols ``*`` that refer to zero or more proceeding symbols.
//! Each star matches its sequential marker number: ``#1, #2, ..., #{10}, #{11}, ... ``
//!
//! \<DESTINATION> contains markers and symbols. Each marker refers to the symbols sequence beyond its ``*``.
//!
//! Prints files that were successfully moved in format:
//!
//! ```console
//! source_dir/filename1 -> destination_dir/filename2
//! ```
//! ## Usage
//!
//! ```console
//! $ mmv 'path/to/some_*_filename.*' 'path2/to/changed_#1_filename.#2'
//! path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
//! path/to/some_A_filename.jpg -> path2/to/changed_A_filename.jpg
//! path/to/some_B_filename.bin -> path2/to/changed_B_filename.bin
//! path/to/some_B_filename.jpg -> path2/to/changed_B_filename.jpg
//! ```
//! *for more information use ``mmv --help``*
#![forbid(unsafe_code)]

use clap::Parser;
use std::{fs, path::Path};

pub mod helpers;
pub mod r#move;

use crate::helpers::get_expression;
use crate::r#move::try_to_move;

#[derive(Parser, Debug)]
/// Command line utility for mass moving or renaming files according to a template.
///
/// Moves files that match template from <SOURCE> directory to <DESTINATION> directory and renames
/// it according to the template. Both arguments shoud contain existing directory path.
/// It is foridden to use '^', '$', '+', '-', '?', '(', ')', '[', ']', '{', '}', '|' in templates.
///
/// <SOURCE> contains a template with symbols '*' that refer to zero or more proceeding symbols.
/// Each star matches its sequential marker number: #1, #2, ..., #{10}, #{11}, ...
///
/// <DESTINATION> contains markers and symbols. Each marker refers to the symbols sequence beyond its '*'.
///
/// Prints files that were successfully moved in format:
///
/// path/to/some_A_filename.bin -> path2/to/changed_A_filename.bin
struct Cli {
    /// Should be string. Pattern of files to move
    source: String,

    /// Should be string. Pattern of destination to move to
    destination: String,

    /// Flag for overwriting files if they exist
    #[arg(short, long)]
    force: bool,
}

/// Reads command line arguments. Looks at all files in the current folder.
/// Tries to move them by calling the try_to_move command.
/// If no files are moved, it throws an error.
pub fn run_pipeline() -> Result<(), String> {
    let cli: Cli = Cli::parse();

    let source_path = Path::new(&cli.source);
    let destination_path = Path::new(&cli.destination);
    let source_template = source_path.file_name().unwrap().to_str().unwrap();
    let expression = get_expression(&source_template)?;
    let mut moved_files = 0;

    for files in fs::read_dir(source_path.parent().unwrap()).unwrap() {
        let file = files.unwrap();
        let sucessfully_moved: bool;
        if !file.path().is_dir() {
            match try_to_move(
                &file.path(),
                &destination_path,
                &expression.as_str(),
                cli.force,
            ) {
                Ok(flag) => sucessfully_moved = flag,
                Err(err) => return Err(err),
            }
            if sucessfully_moved {
                moved_files += 1;
            }
        }
    }
    if moved_files == 0 {
        return Err(["Files for pattern", source_template, "not found."].join(" "));
    }
    Ok(())
}
