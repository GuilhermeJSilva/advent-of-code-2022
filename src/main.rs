use std::env;
mod filecache;
mod client;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let aoc_client =
        client::AocCLient::new(&env::var("AOC_SESSION").expect("Set AOC_SESSION to retrieve inputs"));
    print!("{}", aoc_client.get_input(2021, 1).await);
    Ok(())
}
