use crate::types::DfxResult;
use crate::types::EventLog;
use anyhow::Context;
use anyhow::Result;
use candid::utils::ArgumentDecoder;
use candid::{CandidType, Principal};
use ic_agent::identity::Secp256k1Identity;
use ic_agent::Agent;
use ic_utils::call::SyncCall;
use ic_utils::interfaces::management_canister::{FetchCanisterLogsResponse, MgmtMethod};
use ic_utils::interfaces::ManagementCanister;
use time::OffsetDateTime;

pub const DEFAULT_SHARED_LOCAL_BIND: &str = "127.0.0.1:4943";
pub const DEFAULT_IC_GATEWAY: &str = "https://icp0.io";
pub const DEFAULT_IC_GATEWAY_TRAILING_SLASH: &str = "https://icp0.io/";

fn format_bytes(bytes: &[u8]) -> String {
    format!("(bytes) 0x{}", hex::encode(bytes))
}

pub async fn create_agent(url: &str) -> Result<Agent> {
    let identity = Secp256k1Identity::from_pem_file("tester.pem")?;
    let agent = Agent::builder()
        .with_transport(ic_agent::agent::http_transport::ReqwestTransport::create(
            url,
        )?)
        .with_boxed_identity(Box::new(identity))
        .with_verify_query_signatures(true)
        // .with_ingress_expiry(Some(Duration::from_secs(480)))
        .build()?;

    let is_mainnet = matches!(url, DEFAULT_IC_GATEWAY | DEFAULT_IC_GATEWAY_TRAILING_SLASH);
    if !is_mainnet {
        agent.fetch_root_key().await?;
    }

    Ok(agent)
}

pub async fn get_canister_logs(canister_id: Principal, agent: &Agent) -> Result<Vec<EventLog>> {
    #[derive(CandidType)]
    struct In {
        canister_id: Principal,
    }

    let (out,): (FetchCanisterLogsResponse,) = do_management_query_call(
        canister_id,
        MgmtMethod::FetchCanisterLogs.as_ref(),
        In { canister_id },
        agent,
    )
    .await?;

    let formatted_logs = format_canister_logs(out);
    Ok(formatted_logs)
}

fn format_canister_logs(logs: FetchCanisterLogsResponse) -> Vec<EventLog> {
    logs.canister_log_records
        .into_iter()
        .map(|r| {
            let time = OffsetDateTime::from_unix_timestamp_nanos(r.timestamp_nanos as i128)
                .expect("Invalid canister log record timestamp");

            let message = if let Ok(s) = String::from_utf8(r.content.clone()) {
                if format!("{s:?}").contains("\\u{") {
                    format_bytes(&r.content)
                } else {
                    s
                }
            } else {
                format_bytes(&r.content)
            };

            EventLog::new(
                r.idx, 
                time.unix_timestamp(),
                message
            )


        })
        .collect()
}

async fn do_management_query_call<A, O>(
    destination_canister: Principal,
    method: &str,
    arg: A,
    agent: &Agent,
) -> DfxResult<O>
where
    A: CandidType + Sync + Send,
    O: for<'de> ArgumentDecoder<'de> + Sync + Send,
{
    let mgr = ManagementCanister::create(agent);

    let out = mgr
        .query(method)
        .with_arg(arg)
        .with_effective_canister_id(destination_canister)
        .build()
        .call()
        .await
        .context("Query call failed.")?;

    Ok(out)
}
