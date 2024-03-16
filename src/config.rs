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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::load_config;

    #[test]
    fn test_valid_config() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/config/valid.yaml");

        let config = load_config(path.to_str().unwrap())
            .expect("Failed to load config");

        assert_eq!(config.file_path, "/path/to/ebooks");
        assert_eq!(config.calibre_url.as_str(), "http://calibre.box/");
        assert_eq!(config.acsm_conversion_timeout, 48);
        assert_eq!(config.calibre_upload_timeout, 48);
    }
}

