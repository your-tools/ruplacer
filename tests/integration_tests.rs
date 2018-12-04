extern crate ruplacer;
extern crate tempdir;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempdir::TempDir;

use ruplacer::query;
use ruplacer::DirectoryPatcher;
use ruplacer::Settings;

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
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), Settings::default());
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");

    let top_txt_path = data_path.join("top.txt");
    assert_replaced(&top_txt_path);

    // Also check recursion inside the data dir:
    let foo_path = data_path.join("a_dir/sub/foo.txt");
    assert_replaced(&foo_path);
}

#[test]
fn test_stats() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), Settings::default());

    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");
    let stats = directory_patcher.stats();
    assert_eq!(stats.matching_files, 2);
    assert_eq!(stats.num_replacements, 3);
}

#[test]
fn test_dry_run() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let mut settings = Settings::default();
    settings.dry_run = true;
    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), settings);
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");

    let top_txt_path = data_path.join("top.txt");
    assert_not_replaced(&top_txt_path);
}

#[test]
fn test_with_gitignore() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);

    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), Settings::default());
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");

    let ignored_path = data_path.join(".hidden/hidden.txt");
    assert_not_replaced(&ignored_path);
}

#[test]
fn test_skip_non_utf8_files() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);
    let bin_path = data_path.join("foo.latin1");
    fs::write(bin_path, b"caf\xef\n").unwrap();

    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), Settings::default());
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");
}

#[test]
fn test_select_file_types() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);
    let py_path = data_path.join("foo.py");
    fs::write(py_path, "a = 'this is old'\n").unwrap();

    let mut settings = Settings::default();
    settings.selected_file_types = vec!["py".to_string()];

    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), settings);
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");
    let stats = directory_patcher.stats();
    assert_eq!(stats.matching_files, 1);
}

#[test]
fn test_ignore_file_types() {
    let tmp_dir = TempDir::new("test-ruplacer").expect("failed to create temp dir");
    let data_path = setup_test(&tmp_dir);
    let py_path = data_path.join("foo.py");
    fs::write(&py_path, "a = 'this is old'\n").unwrap();
    let mut settings = Settings::default();
    settings.ignored_file_types = vec!["py".to_string()];
    let mut directory_patcher = DirectoryPatcher::new(data_path.to_path_buf(), settings);
    directory_patcher
        .patch(query::substring("old", "new"))
        .expect("ruplacer failed");
    assert_not_replaced(&py_path);
}
