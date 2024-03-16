use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
pub struct Config {

}

pub fn load_config() -> Result<Config, Box<dyn Error>> {
    Ok(Config {})
}

