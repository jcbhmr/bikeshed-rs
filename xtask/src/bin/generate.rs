fn main() -> Result<(), Box<dyn std::error::Error>> {
    let targets = vec![
        "arm64-apple-darwin",
        "x86_64-apple-darwin",
        "x86_64-pc-windows-msvc",
        "x86_64-unknown-linux-gnu",
    ];
    let bikeshed_builder_version = "4.1.6.1";
    for target in targets.iter() {
        let ext = if target.contains("windows") {
            ".zip"
        } else {
            ".tar.gz"
        };
        let url = format!("https://github.com/jcbhmr/bikeshed-builder/releases/download/v{bikeshed_builder_version}/bikeshed-{target}{ext}");
        let dest = format!("bikeshed-{target}{ext}");
        eprintln!("Downloading {url} to {dest}");
        let mut response = reqwest::blocking::get(&url)?;
        if response.status() != 200 {
            return Err(format!("{url} {}", response.status()).into());
        }
        let mut file = std::fs::File::create(dest)?;
        std::io::copy(&mut response, &mut file)?;
        eprintln!("Success!");
    }
    Ok(())
}
