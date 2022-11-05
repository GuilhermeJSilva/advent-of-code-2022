use serde::{Deserialize, Serialize};
use std::fs;
mod client;
mod filecache;

#[derive(Serialize, Deserialize)]
struct Config {
    aoc_session: String,
}

impl Config {
    fn from_file(path: &str) -> Self {
        let config_str = fs::read_to_string(path).expect("Config file must exist!");
        serde_yaml::from_str(&config_str).expect("Config file does not have the correct format")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_file("config.yml");
    let aoc_client = client::AocCLient::new(&config.aoc_session);
    print!("{}", aoc_client.get_input(2021, 1).await);
    Ok(())
}
