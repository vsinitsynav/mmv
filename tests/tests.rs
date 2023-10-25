#![forbid(unsafe_code)]

use mmv::r#move::try_to_move;
use std::path::Path;

#[test]
fn it_works() {
    let _ = std::fs::File::create("tmp/cbacabc.txt");
    let _ = std::fs::File::create("tmp/bbacabb.txt");
    assert_eq!(
        try_to_move(Path::new("tmp/cbacabc.txt"), Path::new("tmp/a#1a.#3"), "c(.*)(.*)c\\.(.*)", false),
        Ok(true)
    );
    assert_eq!(
        try_to_move(Path::new("tmp/bbacabb.txt"), Path::new("tmp/b#11b.#2"), "a(.*)a\\.(.*)", false),
        Ok(false)
    );
    assert_eq!(
        try_to_move(Path::new("tmp/abacaba.txt"), Path::new("tmp/b#1b.#2"), "a(.*)a\\.(.*)", false),
        Err("Not able to replace existing file: tmp/bbacabb.txt".to_string())
    );
    assert_eq!(
        try_to_move(Path::new("tmp/abacaba.txt"), Path::new("tmp/b#1b.#2"), "a(.*)a\\.(.*)", true),
        Ok(true)
    );

    let _ = std::fs::remove_file("tmp/bbacabb.txt");
    let _ = std::fs::File::create("tmp/some_A_filename.bin");
    let _ = std::fs::File::create("tmp/changed_A_filename.bin");

    assert_eq!(
        try_to_move(Path::new("tmp/some_A_filename.bin"), Path::new("aboba/#1"), "(.*)", false),
        Err("Destination directory does not exist.".to_string())
    );

    assert_eq!(
        try_to_move(
            Path::new("tmp/some_A_filename.bin"),
            Path::new("tmp/changed_#1_filename.#2"),
            "some_(.*)_filename\\.(.*)",
            false
        ),
        Err("Not able to replace existing file: tmp/changed_A_filename.bin".to_string())
    );

    let _ = std::fs::remove_file("tmp/changed_A_filename.bin");
    assert_eq!(
        try_to_move(
            Path::new("tmp/some_A_filename.bin"),
            Path::new("tmp/changed_#1_filename.#2"),
            "some_(.*)_filename\\.(.*)",
            false
        ),
        Ok(true)
    );
    assert!(Path::new("tmp/changed_A_filename.bin").exists());
    let _ = std::fs::remove_file("tmp/changed_A_filename.bin");
}
