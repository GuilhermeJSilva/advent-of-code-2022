use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
mod client;
mod filecache;
mod solutions;

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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    year: u16,
    day: u8,
    #[arg(short, long)]
    part2: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let config = Config::from_file("config.yml");
    let aoc_client = client::AocCLient::new(&config.aoc_session);
    let input = aoc_client.get_input(cli.year, cli.day).await;
    let solutions = solutions::Solutions::new();
    let solution = solutions.get_solution(cli.year, cli.day);
    if cli.part2 {
        println!("{}", solution.solve_part2(input))
    } else {
        println!("{}", solution.solve_part1(input))
    }
    Ok(())
}
