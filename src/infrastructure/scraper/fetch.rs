use anyhow::Result;
use encoding_rs::WINDOWS_874;
use reqwest;
use tracing::info;

pub async fn fetch_html(url: &str) -> Result<String> {
    info!("Fetching HTML from: {}", url);
    let response_bytes = reqwest::get(url).await?.bytes().await?;

    let (decoded_response, _, had_errors) = WINDOWS_874.decode(&response_bytes);

    if had_errors {
        tracing::warn!("Decoding errors occurred while fetching HTML");
    }

    Ok(decoded_response.to_string())
}
