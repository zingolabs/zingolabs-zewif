use crate::test_envelope_roundtrip;
use anyhow::{Context, Result, bail};
use bc_envelope::prelude::*;

/// The status of a transaction in the blockchain
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    /// Transaction is in the mempool, not yet confirmed
    Pending,
    /// Transaction is confirmed in a block
    Confirmed,
    /// Transaction failed to be included in a block
    Failed,
    /// Transaction was abandoned
    Abandoned,
}

impl From<TransactionStatus> for String {
    fn from(value: TransactionStatus) -> Self {
        match value {
            TransactionStatus::Pending => "Pending".to_string(),
            TransactionStatus::Confirmed => "Confirmed".to_string(),
            TransactionStatus::Failed => "Failed".to_string(),
            TransactionStatus::Abandoned => "Abandoned".to_string(),
        }
    }
}

impl TryFrom<String> for TransactionStatus {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "Pending" => Ok(TransactionStatus::Pending),
            "Confirmed" => Ok(TransactionStatus::Confirmed),
            "Failed" => Ok(TransactionStatus::Failed),
            "Abandoned" => Ok(TransactionStatus::Abandoned),
            _ => bail!("Invalid TransactionStatus string"),
        }
    }
}

impl From<TransactionStatus> for Envelope {
    fn from(value: TransactionStatus) -> Self {
        Envelope::new(String::from(value))
    }
}

impl TryFrom<Envelope> for TransactionStatus {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self> {
        let status_str = envelope.extract_subject::<String>().context("TransactionStatus")?;
        TransactionStatus::try_from(status_str)
            .map_err(|_| anyhow::anyhow!("Invalid TransactionStatus envelope"))
    }
}

#[cfg(test)]
impl crate::RandomInstance for TransactionStatus {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rand::Rng::gen_range(&mut rng, 0..4) {
            0 => TransactionStatus::Pending,
            1 => TransactionStatus::Confirmed,
            2 => TransactionStatus::Failed,
            _ => TransactionStatus::Abandoned,
        }
    }
}

test_envelope_roundtrip!(TransactionStatus);
