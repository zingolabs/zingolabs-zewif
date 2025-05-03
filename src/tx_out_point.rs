use super::TxId;
use anyhow::Context;
use bc_envelope::prelude::*;

/// A reference to a specific output from a previous transaction in the Zcash blockchain.
///
/// `TxOutPoint` precisely identifies an output by its transaction ID and index number,
/// functioning as a pointer to a specific UTXO (Unspent Transaction Output) that can
/// be spent as an input in a new transaction.
///
/// # Zcash Concept Relation
/// In Zcash's transparent payment system (similar to Bitcoin's UTXO model):
///
/// - **Transaction ID (txid)**: A unique identifier for the transaction containing the output
/// - **Output Index**: The position of the output in the transaction's list of outputs (zero-based)
///
/// A `TxOutPoint` is used by transaction inputs to reference the specific output they are spending.
/// When a wallet creates a new transaction, it constructs inputs by referencing outputs from
/// previous transactions that it can spend.
///
/// # Data Preservation
/// During wallet migration, the following data is preserved:
///
/// - **Transaction ID**: The complete 32-byte transaction hash
/// - **Output Index**: The exact index number that identifies the specific output
///
/// Together, these components form a unique identifier for a specific output in the blockchain.
///
/// # Examples
/// ```
/// # use zewif::{TxOutPoint, TxId};
/// // Create a transaction ID (normally this would be a real hash)
/// let txid = TxId::from_bytes([0; 32]);
///
/// // Reference the second output (index 1) from that transaction
/// let outpoint = TxOutPoint::new(txid, 1);
///
/// // Access the components
/// assert_eq!(outpoint.index(), 1);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TxOutPoint {
    txid: TxId,
    index: u32,
}

impl TxOutPoint {
    /// Creates a new transaction output reference.
    ///
    /// # Arguments
    /// * `txid` - Transaction ID containing the output
    /// * `index` - Index of the output in the transaction's outputs list (zero-based)
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOutPoint, TxId};
    /// // Create a reference to the first output in a transaction
    /// let txid = TxId::from_bytes([0; 32]);
    /// let outpoint = TxOutPoint::new(txid, 0);
    /// ```
    pub fn new(txid: TxId, index: u32) -> Self {
        Self { txid, index }
    }

    /// Returns the transaction ID containing the referenced output.
    ///
    /// # Returns
    /// The transaction ID as a `TxId`
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOutPoint, TxId};
    /// #
    /// let bytes = [1u8; 32];
    /// let txid = TxId::from_bytes(bytes);
    /// let outpoint = TxOutPoint::new(txid, 0);
    ///
    /// assert_eq!(outpoint.txid(), txid);
    /// ```
    pub fn txid(&self) -> TxId {
        self.txid
    }

    /// Returns the output index within the transaction.
    ///
    /// # Returns
    /// The zero-based index of the output in the transaction
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOutPoint, TxId};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// // Reference the third output (index 2) in a transaction
    /// let outpoint = TxOutPoint::new(txid, 2);
    ///
    /// assert_eq!(outpoint.index(), 2);
    /// ```
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Sets the transaction ID.
    ///
    /// # Arguments
    /// * `txid` - The new transaction ID
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOutPoint, TxId};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// let mut outpoint = TxOutPoint::new(txid, 1);
    ///
    /// // Update to reference a different transaction
    /// let new_txid = TxId::from_bytes([1; 32]);
    /// outpoint.set_txid(new_txid);
    ///
    /// assert_eq!(outpoint.txid(), new_txid);
    /// ```
    pub fn set_txid(&mut self, txid: TxId) {
        self.txid = txid;
    }

    /// Sets the output index.
    ///
    /// # Arguments
    /// * `index` - The new output index
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOutPoint, TxId};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// let mut outpoint = TxOutPoint::new(txid, 0);
    ///
    /// // Update to reference a different output in the same transaction
    /// outpoint.set_index(3);
    ///
    /// assert_eq!(outpoint.index(), 3);
    /// ```
    pub fn set_index(&mut self, index: u32) {
        self.index = index;
    }
}

impl From<TxOutPoint> for Envelope {
    fn from(value: TxOutPoint) -> Self {
        Envelope::new(value.index)
            .add_type("TxOutPoint")
            .add_assertion("txid", value.txid)
    }
}

impl TryFrom<Envelope> for TxOutPoint {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope
            .check_type_envelope("TxOutPoint")
            .context("TxOutPoint")?;
        let index = envelope.extract_subject().context("index")?;
        let txid = envelope
            .extract_object_for_predicate("txid")
            .context("txid")?;

        Ok(TxOutPoint::new(txid, index))
    }
}

#[cfg(test)]
mod tests {
    use crate::{TxId, test_envelope_roundtrip};

    use super::TxOutPoint;

    impl crate::RandomInstance for TxOutPoint {
        fn random() -> Self {
            Self {
                txid: TxId::random(),
                index: rand::random(),
            }
        }
    }

    test_envelope_roundtrip!(TxOutPoint);
}
