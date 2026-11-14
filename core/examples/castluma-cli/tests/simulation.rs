use std::process::Command;

#[test]
fn default_simulation_completes_normal_playback_flow() {
    let output = Command::new(env!("CARGO_BIN_EXE_castluma-cli"))
        .output()
        .expect("failed to start castluma-cli");

    assert!(
        output.status.success(),
        "simulation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("CLI output should be valid UTF-8");

    assert!(stdout.contains("Simulation only: no receiver is contacted"));

    assert!(
        stdout.contains("Playing --PlaybackPaused--> Paused"),
        "missing simulation notice: {stdout}"
    );

    assert!(
        stdout.contains("Stopping --PlaybackStopped--> Ready"),
        "missing final stop transition: {stdout}"
    );
}

#[test]
fn timeout_simulation_records_connection_failure() {
    let output = Command::new(env!("CARGO_BIN_EXE_castluma-cli"))
        .arg("timeout")
        .output()
        .expect("failed to start castluma-cli");

    assert!(
        output.status.success(),
        "timeout simulation failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("CLI output should be valid UTF-8");

    assert!(
        stdout.contains(
            "Connecting --ConnectionFailed(ConnectionTimeout)--> Failed(ConnectionTimeout)"
        ),
        "missing connection failure transition: {stdout}"
    );
}

#[test]
fn unknown_scenario_is_rejected() {
    let output = Command::new(env!("CARGO_BIN_EXE_castluma-cli"))
        .arg("unknown")
        .output()
        .expect("failed to start castluma-cli");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("CLI error output should be valid UTF-8");

    assert!(
        stderr.contains("Usage: castluma-cli [normal|timeout]"),
        "missing usage message: {stderr}"
    );
}
