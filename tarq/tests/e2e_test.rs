use std::fs::{self};
use std::process::{Command, Stdio};

#[test]
fn e2e_test() {
    let child = Command::new("cargo")
        .arg("run")
        .arg("tests/samples/unit/index.js")
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    let file_content = fs::read_to_string("output.json").unwrap();
    assert_eq!(file_content, "{\n  \"class_number\": 4\n}");
    assert!(child.status.success());
}
