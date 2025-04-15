use bc_envelope::prelude::*;
use crate::{test_envelope_roundtrip, Indexed};
use super::{Amount, Script};
use anyhow::Context;

/// A transparent transaction output in a Zcash transaction.
///
/// `TxOut` represents an output from a transparent transaction, consisting of a ZEC amount
/// and a script that defines the conditions under which the output can be spent. These outputs
/// become UTXOs (Unspent Transaction Outputs) that can later be referenced as inputs in new
/// transactions.
///
/// # Zcash Concept Relation
/// In Zcash's transparent payment system (inherited from Bitcoin):
///
/// - **Value**: The amount of ZEC encoded in zatoshi (1 ZEC = 10^8 zatoshi)
/// - **Script Public Key**: Defines the conditions required for spending this output
///   - May contain public key hashes (P2PKH), script hashes (P2SH), or other script types
///   - Establishes the "lock" that must be "unlocked" by a corresponding script signature
///
/// Transparent outputs are publicly visible on the blockchain, unlike shielded outputs which
/// encrypt their details using zero-knowledge proofs.
///
/// # Data Preservation
/// During wallet migration, the following data is preserved:
///
/// - **Amount**: The exact ZEC amount of the output, down to the zatoshi
/// - **Script**: The complete script that defines the spending conditions, preserved byte-for-byte
///
/// # Examples
/// ```
/// # use zewif::{TxOut, Amount, Script, Data};
/// # use anyhow::Result;
/// # fn example() -> Result<()> {
/// // Create a script (typically a P2PKH script containing a public key hash)
/// // Standard P2PKH format: OP_DUP(0x76), OP_HASH160(0xa9), pushbytes_20(0x14), <hash>, OP_EQUALVERIFY(0x88), OP_CHECKSIG(0xac)
/// let script_bytes = vec![0x76, 0xa9, 0x14, 0x01, 0x02, 0x03, 0x04, 0x88, 0xac];
/// let script_pubkey = Script::from(Data::from_vec(script_bytes));
///
/// // Create an output of 1.5 ZEC with the script
/// let amount = Amount::from_u64(150000000)?; // 1.5 ZEC in zatoshi
/// let tx_out = TxOut::new(amount, script_pubkey);
///
/// // Access the components
/// let value: i64 = (*tx_out.value()).into();
/// assert_eq!(value, 150000000);
/// assert!(!tx_out.script_pubkey().is_empty());
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TxOut {
    index: usize,
    value: Amount,
    script_pubkey: Script,
}

impl Indexed for TxOut {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl TxOut {
    /// Creates a new transaction output.
    ///
    /// # Arguments
    /// * `value` - The amount of ZEC in this output
    /// * `script_pubkey` - Script defining the spending conditions
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOut, Amount, Script, Data};
    /// # use anyhow::Result;
    /// # fn example() -> Result<()> {
    /// // Create a simple script (normally this would be a P2PKH or P2SH script)
    /// let script_bytes = vec![0x76, 0xa9, 0x14, /* pubkey hash would go here */];
    /// let script_pubkey = Script::from(Data::from_vec(script_bytes));
    ///
    /// // Create an output worth 0.5 ZEC
    /// let amount = Amount::from_u64(50000000)?; // In zatoshi (1 ZEC = 10^8 zatoshi)
    /// let tx_out = TxOut::new(amount, script_pubkey);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(value: Amount, script_pubkey: Script) -> Self {
        Self {
            index: 0, // Default index, can be set later
            value,
            script_pubkey,
        }
    }

    /// Returns a reference to the output value.
    ///
    /// # Returns
    /// Reference to the `Amount` contained in this output
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOut, Amount, Script, Data};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let script_pubkey = Script::from(Data::from_vec(vec![0x00]));
    /// # let amount = Amount::from_u64(10000000)?; // 0.1 ZEC
    /// # let tx_out = TxOut::new(amount, script_pubkey);
    /// #
    /// let output_amount = tx_out.value();
    /// let zatoshi: i64 = (*output_amount).into();
    /// assert_eq!(zatoshi, 10000000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn value(&self) -> &Amount {
        &self.value
    }

    /// Returns a reference to the script public key.
    ///
    /// # Returns
    /// Reference to the `Script` that defines spending conditions for this output
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOut, Amount, Script, Data};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let script_bytes = vec![0x76, 0xa9, 0x14, 0x01, 0x02, 0x03, 0x04];
    /// # let script_pubkey = Script::from(Data::from_vec(script_bytes.clone()));
    /// # let amount = Amount::from_u64(0)?;
    /// # let tx_out = TxOut::new(amount, script_pubkey);
    /// #
    /// let script = tx_out.script_pubkey();
    ///
    /// // Check the script contains the expected bytes
    /// let script_data: &[u8] = script.as_ref();
    /// assert_eq!(script_data, script_bytes.as_slice());
    /// # Ok(())
    /// # }
    /// ```
    pub fn script_pubkey(&self) -> &Script {
        &self.script_pubkey
    }

    /// Sets the output value.
    ///
    /// # Arguments
    /// * `value` - The new ZEC amount for this output
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOut, Amount, Script, Data};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let script_pubkey = Script::from(Data::from_vec(vec![0x00]));
    /// # let amount = Amount::from_u64(0)?;
    /// # let mut tx_out = TxOut::new(amount, script_pubkey);
    /// #
    /// // Change the output amount to 2 ZEC
    /// let new_amount = Amount::from_u64(200000000)?;
    /// tx_out.set_value(new_amount);
    ///
    /// let zatoshi: i64 = (*tx_out.value()).into();
    /// assert_eq!(zatoshi, 200000000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    /// Sets the script public key.
    ///
    /// # Arguments
    /// * `script_pubkey` - The new script defining spending conditions
    ///
    /// # Examples
    /// ```
    /// # use zewif::{TxOut, Amount, Script, Data};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let script_pubkey = Script::from(Data::from_vec(vec![0x00]));
    /// # let amount = Amount::from_u64(0)?;
    /// # let mut tx_out = TxOut::new(amount, script_pubkey);
    /// #
    /// // Create a new P2PKH-like script
    /// let new_script_bytes = vec![0x76, 0xa9, 0x14, /* pubkey hash */];
    /// let new_script = Script::from(Data::from_vec(new_script_bytes));
    ///
    /// // Update the output with the new script
    /// tx_out.set_script_pubkey(new_script);
    ///
    /// assert_eq!(tx_out.script_pubkey().len(), 3);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_script_pubkey(&mut self, script_pubkey: Script) {
        self.script_pubkey = script_pubkey;
    }
}

impl From<TxOut> for Envelope {
    fn from(value: TxOut) -> Self {
        Envelope::new(value.index)
            .add_type("TxOut")
            .add_assertion("value", value.value)
            .add_assertion("script_pubkey", value.script_pubkey)
    }
}

impl TryFrom<Envelope> for TxOut {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("TxOut").context("TxOut")?;
        let index = envelope.extract_subject().context("index")?;
        let value = envelope.extract_object_for_predicate("value").context("value")?;
        let script_pubkey = envelope.extract_object_for_predicate("script_pubkey").context("script_pubkey")?;
        let mut tx_out = TxOut::new(value, script_pubkey);
        tx_out.set_index(index);
        Ok(tx_out)
    }
}

#[cfg(test)]
impl crate::RandomInstance for TxOut {
    fn random() -> Self {
        Self {
            index: usize::random(),
            value: Amount::random(),
            script_pubkey: Script::random(),
        }
    }
}

test_envelope_roundtrip!(TxOut);
