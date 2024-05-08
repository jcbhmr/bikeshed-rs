#!/usr/bin/env -S cargo +nightly -Zscript
```cargo
[dependencies]
reqwest = { version = "0.12.4", features = ["blocking"] }
```
fn generate() -> Result<(), Box<dyn std::error::Error>> {
    let targets = vec![
        "arm64-apple-darwin",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu"
    ];
    for target in targets.iter() {
        println!("{}", target)
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    match std::env::args().nth(1).ok_or("no task")?.as_str() {
        "generate" => generate(),
        _ => Err("no such task".into()),
    }
}