use std::str::FromStr;

use candid::Principal;
use config::Config;
use dotenv::dotenv;
use ic::{create_agent, get_canister_logs};

pub mod config;
pub mod errors;
pub mod ic;
pub mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let config = Config::default();
    let agent = config.get_agent().await.unwrap();

    let logs = get_canister_logs(
        config.canister,
        &agent,
    ).await.unwrap();

    println!("{:?}",logs);
    Ok(())
}
