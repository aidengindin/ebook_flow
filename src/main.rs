use crate::config::load_config;

fn main() {
    let config = if let Ok(config) = load_config() {
        config
    } else {
        eprintln!("Failed to load configuration!");
        std::process::exit(1);
    };
}

