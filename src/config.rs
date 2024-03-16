use serde::Deserialize;
use std::error::Error;
use url::Url;

#[derive(Deserialize)]
pub struct Config {
    file_path: String,
    calibre_url: Url,
    acsm_conversion_timeout: u32,
    calibre_upload_timeout: u32,
}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    Ok(Config {
        file_path: "".to_string(),
        calibre_url: Url::parse("https://aidengindin.com")?,
        acsm_conversion_timeout: 48,
        calibre_upload_timeout: 48,
    })
}

