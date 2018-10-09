extern crate ruplacer;
extern crate tempdir;
use ruplacer::Replacer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempdir::TempDir;

fn setup_test(tmp_dir: &TempDir) -> PathBuf {
    let tmp_path = tmp_dir.path();
    let status = Command::new("cp")
        .args(&["-R", "tests/data", &tmp_path.to_string_lossy()])
        .status()
        .expect("Failed to execute process");
    assert!(status.success());
    tmp_path.join("data")
}

fn assert_replaced(path: &Path) {
    let contents = fs::read_to_string(&path).expect(&format!("Could not read from {:?}", path));
    assert!(contents.contains("new"));
    assert!(!contents.contains("old"));
}

fn assert_not_replaced(path: &Path) {
    let contents = fs::read_to_string(&path).expect(&format!("Could not read from {:?}", path));
    assert!(!contents.contains("new"));
    assert!(contents.contains("old"));
}

#[test]
fn test_replace_old_by_new() {
    let tmp_dir = TempDir::new("test-replacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let replacer = Replacer::new(data_path.to_path_buf());
    replacer.replace("old", "new").expect("replacer failed");

    let top_txt_path = data_path.join("top.txt");
    assert_replaced(&top_txt_path);
}

#[test]
fn test_with_gitignore() {
    let tmp_dir = TempDir::new("test-replacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let replacer = Replacer::new(data_path.to_path_buf());
    replacer.replace("old", "new").expect("replacer failed");

    let ignored_path = data_path.join(".hidden/hidden.txt");
    assert_not_replaced(&ignored_path);
}
