use std::process::Command;

fn stardial() -> Command {
    Command::new(env!("CARGO_BIN_EXE_stardial"))
}

#[test]
fn help_exits_zero() {
    let output = stardial().arg("--help").output().unwrap();
    assert!(output.status.success(), "--help should exit 0");
}

#[test]
fn version_exits_zero() {
    let output = stardial().arg("--version").output().unwrap();
    assert!(output.status.success(), "--version should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("stardial"),
        "version output should contain binary name"
    );
}

#[test]
fn invalid_option_exits_nonzero() {
    let output = stardial().arg("--no-such-flag").output().unwrap();
    assert!(
        !output.status.success(),
        "invalid option should exit non-zero"
    );
}

#[test]
fn invalid_fps_range_exits_nonzero() {
    let output = stardial().args(["--fps", "999"]).output().unwrap();
    assert!(
        !output.status.success(),
        "out-of-range --fps should exit non-zero"
    );
}
