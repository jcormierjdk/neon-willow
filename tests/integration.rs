use assert_cmd::Command;

#[test]
fn test_no_args() {
    Command::cargo_bin("neon_willow")
        .unwrap()
        .assert()
        .success();
}