#[test]
fn version() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new(env!("CARGO_BIN_EXE_bikeshed"))
        .arg("--version")
        .status()?
        .success()
        .then_some(())
        .ok_or("cmd failed".into())
}
