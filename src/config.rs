use serde::Deserialize;
use serde_yaml;
use url::Url;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct Config {
    file_path: String,
    calibre_url: Url,
    acsm_conversion_timeout: u32,
    calibre_upload_timeout: u32,
}

pub fn load_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

