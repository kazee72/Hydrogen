use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;
use std::fs;

#[test]
fn file_roundtrip() {
    let mut input = NamedTempFile::new().unwrap();
    input.write_all(b"test for file rountrip").unwrap();

    let mut cmd = Command::cargo_bin("hydrogen").expect("binary not found");
    cmd.args(&["compress", input.path().to_str().unwrap()]);

    cmd.assert().success();

    let mut cmd2 = Command::cargo_bin("hydrogen").expect("binary not found");
    let input_h2_path = input.path().with_extension("h2").to_string_lossy().to_string();
    let output_path = input.path().with_extension("txt").to_string_lossy().to_string();

    cmd2.args(&["decompress", "-o", &output_path, &input_h2_path]);

    cmd2.assert().success();

    let original_input = fs::read(input.path()).unwrap();
    let decompressed_input = fs::read(&output_path).unwrap();

    assert_eq!(original_input, decompressed_input);
}