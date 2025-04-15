use super::{Script, TxOutPoint};
use crate::Indexed;
use crate::test_envelope_roundtrip;
use anyhow::Context;
use bc_envelope::prelude::*;

/// A transparent transaction input in a Zcash transaction.
///
/// `TxIn` represents an input to a transparent transaction, consisting of a reference
/// to a previous output being spent, a script signature that satisfies the spending conditions,
/// and a sequence number used for transaction replacement and timelock features.
///
/// # Zcash Concept Relation
/// In Zcash's transparent address system (inherited from Bitcoin):
///
/// - **Previous Output**: Points to a specific UTXO (unspent transaction output) being spent
/// - **Script Signature**: Contains data (signatures, public keys, etc.) that satisfies the
///   spending conditions defined in the previous output's script
/// - **Sequence Number**: Controls transaction finality and timelocks, allowing for features
///   like replace-by-fee and relative timelock
///
/// In contrast to shielded transactions which use zero-knowledge proofs, transparent transactions
/// use these explicit inputs and scripts that are visible on the blockchain.
///
/// # Data Preservation
/// During wallet migration, the following components of a transaction input are preserved:
///
/// - **Transaction Reference**: The exact transaction ID and output index being spent
/// - **Script Signatures**: The complete script that proves spending authority
/// - **Sequence Value**: The precise sequence number with its semantic meaning
///
/// # Examples
/// ```
/// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
/// // Create a TxOutPoint referencing output #1 of a transaction
/// let txid = TxId::from_bytes([0; 32]);
/// let prev_out = TxOutPoint::new(txid, 1);
///
/// // Create a script signature (typically contains signatures and public keys)
/// // Signature (0x47) followed by public key (0x21)
/// let script_bytes = vec![0x47, 0x21];
/// let script_sig = Script::from(Data::from_vec(script_bytes));
///
/// // Create the transaction input with a standard sequence value
/// let tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
///
/// // Access the components
/// assert_eq!(tx_in.previous_output().index(), 1);
/// assert!(!tx_in.script_sig().is_empty());
/// assert_eq!(tx_in.sequence(), 0xffffffff);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TxIn {
    index: usize,
    previous_output: TxOutPoint,
    /// Script signature for unlocking the previous output.
    script_sig: Script,
    sequence: u32,
}

impl Indexed for TxIn {
    /// Returns the index of the output being spent.
    fn index(&self) -> usize {
        self.index
    }

    /// Sets the index of the output being spent.
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl TxIn {
    /// Creates a new transaction input.
    ///
    /// # Arguments
    /// * `previous_output` - Reference to the transaction output being spent
    /// * `script_sig` - Script that satisfies the spending conditions
    /// * `sequence` - Sequence number for transaction replacement and timelocks
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// let txid = TxId::from_bytes([0; 32]);
    /// let prev_out = TxOutPoint::new(txid, 0);
    /// let script_sig = Script::from(Data::from_vec(vec![0x00])); // Empty script
    ///
    /// // Create a transaction input with maximum sequence
    /// let tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// ```
    pub fn new(previous_output: TxOutPoint, script_sig: Script, sequence: u32) -> Self {
        Self {
            index: 0, // Default index, can be set later
            previous_output,
            script_sig,
            sequence,
        }
    }

    /// Returns a reference to the previous output being spent.
    ///
    /// # Returns
    /// Reference to the `TxOutPoint` identifying the output being spent
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 1);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x00]));
    /// # let tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// #
    /// let out_point = tx_in.previous_output();
    /// assert_eq!(out_point.index(), 1);
    /// ```
    pub fn previous_output(&self) -> &TxOutPoint {
        &self.previous_output
    }

    /// Returns a reference to the script signature.
    ///
    /// # Returns
    /// Reference to the `Script` containing the signature data that satisfies
    /// the spending conditions of the previous output
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 0);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x48, 0x30, 0x45, 0x02]));
    /// # let tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// #
    /// let sig = tx_in.script_sig();
    /// assert_eq!(sig.len(), 4); // Length of our example script
    /// ```
    pub fn script_sig(&self) -> &Script {
        &self.script_sig
    }

    /// Returns the sequence number.
    ///
    /// # Returns
    /// The 32-bit sequence number used for transaction replacement and timelock features
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 0);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x00]));
    /// let tx_in = TxIn::new(prev_out, script_sig, 0xfffffffe);
    ///
    /// assert_eq!(tx_in.sequence(), 0xfffffffe);
    /// ```
    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    /// Sets the previous output reference.
    ///
    /// # Arguments
    /// * `previous_output` - New reference to the transaction output being spent
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 0);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x00]));
    /// # let mut tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// #
    /// // Create a new outpoint to a different output
    /// let new_txid = TxId::from_bytes([1; 32]);
    /// let new_prev_out = TxOutPoint::new(new_txid, 2);
    ///
    /// // Update the input to spend this different output
    /// tx_in.set_previous_output(new_prev_out);
    ///
    /// assert_eq!(tx_in.previous_output().index(), 2);
    /// ```
    pub fn set_previous_output(&mut self, previous_output: TxOutPoint) {
        self.previous_output = previous_output;
    }

    /// Sets the script signature.
    ///
    /// # Arguments
    /// * `script_sig` - New script that satisfies the spending conditions
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 0);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x00]));
    /// # let mut tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// #
    /// // Create a new, non-empty script signature
    /// let new_script = Script::from(Data::from_vec(vec![0x48, 0x30, 0x45, 0x02]));
    ///
    /// // Update the input with the new script signature
    /// tx_in.set_script_sig(new_script);
    ///
    /// assert_eq!(tx_in.script_sig().len(), 4);
    /// ```
    pub fn set_script_sig(&mut self, script_sig: Script) {
        self.script_sig = script_sig;
    }

    /// Sets the sequence number.
    ///
    /// # Arguments
    /// * `sequence` - New sequence number for the input
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxIn, TxOutPoint, TxId, Script, Data};
    /// #
    /// # let txid = TxId::from_bytes([0; 32]);
    /// # let prev_out = TxOutPoint::new(txid, 0);
    /// # let script_sig = Script::from(Data::from_vec(vec![0x00]));
    /// # let mut tx_in = TxIn::new(prev_out, script_sig, 0xffffffff);
    /// #
    /// // Set a sequence value that enables replace-by-fee
    /// let rbf_sequence = 0xffffffff - 2;
    /// tx_in.set_sequence(rbf_sequence);
    ///
    /// assert_eq!(tx_in.sequence(), 0xfffffffd);
    /// ```
    pub fn set_sequence(&mut self, sequence: u32) {
        self.sequence = sequence;
    }
}

impl From<TxIn> for Envelope {
    fn from(value: TxIn) -> Self {
        Envelope::new(value.index)
            .add_type("TxIn")
            .add_assertion("previous_output", value.previous_output)
            .add_assertion("script_sig", value.script_sig)
            .add_assertion("sequence", value.sequence)
    }
}

impl TryFrom<Envelope> for TxIn {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("TxIn").context("TxIn")?;
        let index = envelope.extract_subject().context("index")?;
        let previous_output = envelope.try_object_for_predicate("previous_output").context("previous_output")?;
        let script_sig = envelope.extract_object_for_predicate("script_sig").context("script_sig")?;
        let sequence = envelope.extract_object_for_predicate("sequence").context("sequence")?;
        let mut tx_in = TxIn::new(previous_output, script_sig, sequence);
        tx_in.set_index(index);
        Ok(tx_in)
    }
}

#[cfg(test)]
impl crate::RandomInstance for TxIn {
    fn random() -> Self {
        Self {
            index: usize::random(),
            previous_output: TxOutPoint::random(),
            script_sig: Script::random(),
            sequence: u32::random(),
        }
    }
}

test_envelope_roundtrip!(TxIn);
