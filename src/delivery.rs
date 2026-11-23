use anyhow::{ensure, Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
    time::Duration,
};

pub fn save(download: &Value, output: &Path) -> Result<Value> {
    ensure!(!output.exists(), "Output already exists");
    let url = download["url"].as_str().context("Missing download URL")?;
    let parsed = url::Url::parse(url)?;
    ensure!(
        parsed.scheme() == "https"
            || (parsed.scheme() == "http"
                && matches!(parsed.host_str(), Some("127.0.0.1" | "localhost"))),
        "Download requires HTTPS or loopback"
    );
    let size = download["sizeBytes"]
        .as_u64()
        .context("Missing download size")?;
    ensure!((1..=536_870_912).contains(&size), "Invalid delivery size");
    let expected = download["sha256"].as_str().context("Missing SHA-256")?;
    // This separate client never forwards session credentials to signed object URLs.
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(300))
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let response = client
        .get(url)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(reqwest::Error::without_url)
        .context("Could not download delivery")?;
    let mut bytes = Vec::new();
    response.take(size + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 == size, "Delivery size mismatch");
    let sha256 = format!("{:x}", Sha256::digest(&bytes));
    ensure!(sha256 == expected, "Delivery SHA-256 mismatch");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    let written = file.write_all(&bytes).and_then(|()| file.sync_all());
    drop(file);
    if let Err(error) = written {
        std::fs::remove_file(output)
            .context("Could not remove incomplete delivery after a write failure")?;
        return Err(error).context("Could not save delivery");
    }
    Ok(serde_json::json!({"path":output,"bytes":size,"sha256":sha256,"verified":true}))
}
