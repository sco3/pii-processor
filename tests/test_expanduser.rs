use ductaper::config::expanduser::expand_user_path;
use home::home_dir;
use std::env::current_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[test]
fn test_expand_user_path() {
    // Step 1: Create temp file "asdf" in current directory
    let file_name = "temp_test_file.txt";
    let mut file = File::create(file_name).expect("Temp file create error");
    writeln!(file, "asdf").expect("Failed to write to temp file");

    // Step 2: Calculate ~/ path to the file
    let current_dir = current_dir().expect("Failed to get current directory");
    let home_dir = home_dir().expect("Failed to get home directory");

    // Ensure current_dir starts with home_dir
    assert!(
        current_dir.starts_with(&home_dir),
        "Current directory must be inside home directory for this test to work"
    );

    // Get relative path from home to temp file
    let rel_path = current_dir.strip_prefix(&home_dir).unwrap().join(file_name);
    let user_path = format!("~/{}", rel_path.to_string_lossy());

    assert!(
        user_path.starts_with("~/"),
        "User path must start with ~/ for this test to work"
    );
    // Step 3: Expand user path
    let resolved_path = expand_user_path(&user_path);

    // Step 4: Assert file exists at resolved path
    assert!(
        resolved_path.exists(),
        "Resolved path does not exist: {:?}",
        resolved_path
    );

    // Step 5: Clean up
    fs::remove_file(file_name).expect("Failed to delete temp file");
    assert_eq!(expand_user_path("/tmp"), PathBuf::from("/tmp"));
}
