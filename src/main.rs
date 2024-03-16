mod config;
mod conversion;
mod file_operations;
mod upload;

use crate::config::load_config;

fn main() {
    let config_path = "./config.yaml";
    let config = if let Ok(config) = load_config(config_path) {
        config
    } else {
        eprintln!("Failed to load configuration!");
        std::process::exit(1);
    };
}

