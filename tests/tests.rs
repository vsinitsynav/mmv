#![forbid(unsafe_code)]

use mmv::helpers::{create_marker, get_expression, match_source_pattern};
use mmv::r#move::try_to_move;
use std::path::Path;
use tempdir::TempDir;

#[test]
fn it_works() {
    let tmp_dir = TempDir::new("tmp").unwrap();
    let _ = std::fs::File::create(tmp_dir.path().join("cbacabc.txt"));
    let _ = std::fs::File::create(tmp_dir.path().join("bbacabb.txt"));
    assert!(try_to_move(
        &tmp_dir.path().join("cbacabc.txt"),
        &tmp_dir.path().join("a#1a.#3"),
        "c(.*)(.*)c\\.(.*)",
        false
    )
    .is_ok_and(|f| f == true));
    assert!(try_to_move(
        &tmp_dir.path().join("bbacabb.txt"),
        &tmp_dir.path().join("b#11b.#2"),
        "a(.*)a\\.(.*)",
        false
    )
    .is_ok_and(|f| f == false));

    assert!(try_to_move(
        &tmp_dir.path().join("abacaba.txt"),
        &tmp_dir.path().join("b#1b.#2"),
        "a(.*)a\\.(.*)",
        false
    )
    .is_err_and(|error| error.to_string()
        == "Not able to replace existing file: \"".to_string()
            + (tmp_dir.path().join("bbacabb.txt").to_str().unwrap())
            + "\""));
    assert!(try_to_move(
        &tmp_dir.path().join("abacaba.txt"),
        &tmp_dir.path().join("b#1b.#2"),
        "a(.*)a\\.(.*)",
        true
    )
    .is_ok_and(|f| f == true));

    let _ = std::fs::remove_file(tmp_dir.path().join("bbacabb.txt"));
    let _ = std::fs::File::create(tmp_dir.path().join("some_A_filename.bin"));
    let _ = std::fs::File::create(tmp_dir.path().join("changed_A_filename.bin"));

    assert!(try_to_move(
        &tmp_dir.path().join("some_A_filename.bin"),
        Path::new("aboba/#1"),
        "(.*)",
        false
    )
    .is_err_and(|error| error.to_string() == "Destination directory does not exist."));

    assert!(try_to_move(
        &tmp_dir.path().join("some_A_filename.bin"),
        &tmp_dir.path().join("changed_#1_filename.#2"),
        "some_(.*)_filename\\.(.*)",
        false
    )
    .is_err_and(|error| error.to_string()
        == "Not able to replace existing file: \"".to_string()
            + (tmp_dir
                .path()
                .join("changed_A_filename.bin")
                .to_str()
                .unwrap())
            + "\""));

    let _ = std::fs::remove_file(&tmp_dir.path().join("changed_A_filename.bin"));
    assert!(try_to_move(
        &tmp_dir.path().join("some_A_filename.bin"),
        &tmp_dir.path().join("changed_#1_filename.#2"),
        "some_(.*)_filename\\.(.*)",
        false
    )
    .is_ok_and(|f| f == true));
    assert!(&tmp_dir.path().join("changed_A_filename.bin").exists());
    let _ = std::fs::remove_file(tmp_dir.path().join("changed_A_filename.bin"));

    let _ = tmp_dir.close();
}

#[test]
fn test_match_source_pattern() {
    let expression = "a(.*)a";
    let name_1 = "abacaba";
    let name_2 = "baa";

    assert!(match_source_pattern(expression, name_1));
    assert!(!match_source_pattern(expression, name_2));
}

#[test]
fn test_get_expression() {
    assert_eq!(get_expression("*"), Ok("(.*)".to_string()));
    assert_eq!(
        get_expression("Hello, World!"),
        Ok("Hello, World!".to_string())
    );
    assert_eq!(
        get_expression("a*b*c.d**d.txt"),
        Ok("a(.*)b(.*)c\\.d(.*)(.*)d\\.txt".to_string())
    );
    assert_eq!(
        get_expression("Hello, World?"),
        Err("Special characters are not allowed in the source template".to_string())
    );
}

#[test]
fn test_create_marker() {
    assert_eq!(create_marker(1), "#1");
    assert_eq!(create_marker(19), "#{19}");
    assert_eq!(create_marker(1337), "#{1337}");
}
