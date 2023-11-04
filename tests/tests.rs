#![forbid(unsafe_code)]

use mmv::r#move::try_to_move;
use std::path::Path;
use tempdir::TempDir;

#[test]
fn it_works() {
    let tmp_dir = TempDir::new("tmp").unwrap();
    let _ = std::fs::File::create(tmp_dir.path().join("cbacabc.txt"));
    let _ = std::fs::File::create(tmp_dir.path().join("bbacabb.txt"));
    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("cbacabc.txt"),
            &tmp_dir.path().join("a#1a.#3"),
            "c(.*)(.*)c\\.(.*)",
            false
        ),
        Ok(true)
    );
    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("bbacabb.txt"),
            &tmp_dir.path().join("b#11b.#2"),
            "a(.*)a\\.(.*)",
            false
        ),
        Ok(false)
    );
    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("abacaba.txt"),
            &tmp_dir.path().join("b#1b.#2"),
            "a(.*)a\\.(.*)",
            false
        ),
        Err("Not able to replace existing file: ".to_string()
            + (tmp_dir.path().join("bbacabb.txt").to_str().unwrap()))
    );
    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("abacaba.txt"),
            &tmp_dir.path().join("b#1b.#2"),
            "a(.*)a\\.(.*)",
            true
        ),
        Ok(true)
    );

    let _ = std::fs::remove_file(tmp_dir.path().join("bbacabb.txt"));
    let _ = std::fs::File::create(tmp_dir.path().join("some_A_filename.bin"));
    let _ = std::fs::File::create(tmp_dir.path().join("changed_A_filename.bin"));

    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("some_A_filename.bin"),
            Path::new("aboba/#1"),
            "(.*)",
            false
        ),
        Err("Destination directory does not exist.".to_string())
    );

    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("some_A_filename.bin"),
            &tmp_dir.path().join("changed_#1_filename.#2"),
            "some_(.*)_filename\\.(.*)",
            false
        ),
        Err("Not able to replace existing file: ".to_string()
            + (tmp_dir
                .path()
                .join("changed_A_filename.bin")
                .to_str()
                .unwrap()))
    );

    let _ = std::fs::remove_file(&tmp_dir.path().join("changed_A_filename.bin"));
    assert_eq!(
        try_to_move(
            &tmp_dir.path().join("some_A_filename.bin"),
            &tmp_dir.path().join("changed_#1_filename.#2"),
            "some_(.*)_filename\\.(.*)",
            false
        ),
        Ok(true)
    );
    assert!(&tmp_dir.path().join("changed_A_filename.bin").exists());
    let _ = std::fs::remove_file(tmp_dir.path().join("changed_A_filename.bin"));

    let _ = tmp_dir.close();
}
