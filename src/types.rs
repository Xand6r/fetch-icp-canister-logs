use candid::Principal;

use crate::errors::CallSenderFromWalletError;

/// The type to represent DFX results.
pub type DfxResult<T = ()> = anyhow::Result<T>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum CallSender {
    SelectedId,
    Wallet(Principal),
}

// Determine whether the selected Identity or a wallet should be the sender of the call.
// If a wallet, the principal can be selected directly, or looked up from an identity name.
impl CallSender {
    pub fn from(
        wallet_principal_or_identity_name: &Option<String>,
    ) -> Result<Self, CallSenderFromWalletError> {
        let sender = if let Some(s) = wallet_principal_or_identity_name {
            match Principal::from_text(s) {
                Ok(principal) => CallSender::Wallet(principal),
                Err(principal_err) => {
                    return Err(CallSenderFromWalletError::ParsePrincipalFromIdFailed(
                        s.clone(),
                        principal_err,
                    ));
                }
            }
        } else {
            CallSender::SelectedId
        };
        Ok(sender)
    }
}

#[derive(Debug)]
pub struct EventLog {
    pub index: u64,
    pub timestamp: i64,
    pub logs: String,
}

impl EventLog {
    pub fn new(index: u64, timestamp: i64, logs: String) -> Self {
        Self {
            index,
            timestamp,
            logs,
        }
    }
}
