use anyhow::Result;
use encoding_rs::WINDOWS_874;
use reqwest;

pub async fn fetch_html(url: &str) -> Result<String> {
    let response_bytes = reqwest::get(url).await?.bytes().await?;
    let (decoded_response, _, had_errors) = WINDOWS_874.decode(&response_bytes);

    if had_errors {
        eprintln!("Warning: Decoding errors occurred.");
    }

    Ok(decoded_response.to_string())
}
