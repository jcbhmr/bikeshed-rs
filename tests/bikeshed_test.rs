const BIKESHED: &str = env!("CARGO_BIN_EXE_bikeshed");

#[test]
fn version() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new(BIKESHED)
        .arg("--version")
        .status()?
        .success()
        .then_some(())
        .ok_or("cmd failed".into())
}

#[test]
fn help() -> Result<(), Box<dyn std::error::Error>> {
    std::process::Command::new(BIKESHED)
        .arg("--help")
        .status()?
        .success()
        .then_some(())
        .ok_or("cmd failed".into())
}
