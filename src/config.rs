use std::str::FromStr;

use anyhow::Result;
use candid::Principal;
use ic_agent::Agent;
use serde::{Deserialize, Serialize};

use crate::ic::create_agent;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// The URL of the ICP server to connect to
    pub url: String,
    /// The Canister's principal
    pub canister: Principal,
    /// The path to the pem keyfile to generate an identity from
    pub keyfile_path: String
}

impl Default for Config {
    // TODO: get this from a config file rather than an env file
    fn default() -> Self {
        let icp_url = std::env::var("ICP_URL").expect("ICP_URL must be set.");
        let canister_principal = std::env::var("CANISTER_PRINCIPAL").expect("CANISTER_PRINCIPAL must be set.");
        let canister_principal =
            Principal::from_str(&canister_principal).expect("invalid CANISTER_PRINCIPAL");
        let keyfile_path = std::env::var("KEYFILE_PATH").expect("KEYFILE_PATH must be set.");

        Self {
            url: icp_url,
            canister: canister_principal,
            keyfile_path: keyfile_path
        }
    }
}

impl Config{
    pub async fn get_agent(&self) -> Result<Agent> {
        let agent = create_agent(&self.url).await?;
        Ok(agent)
    }
}
