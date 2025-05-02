use anyhow::Context;
use bc_envelope::prelude::*;

use crate::{Amount, Blob, Indexed, Memo};

/// Represents a sent output in an Orchard shielded transaction within a Zcash wallet.
///
/// `OrchardSentOutput` stores the plaintext details of an Orchard note that was sent by the
/// wallet, which are not recoverable from the blockchain after transmission. This information
/// enables selective disclosure, allowing a sender to prove they made a payment to a specific
/// shielded address without revealing additional transaction details.
///
/// # Zcash Concept Relation
/// In Zcash's Orchard protocol (introduced in Network Upgrade 5):
///
/// - **Orchard** represents the next-generation shielded protocol after Sapling
/// - **Improved cryptography** using the Pallas elliptic curve (instead of Jubjub in Sapling)
/// - **Enhanced privacy** with design improvements over earlier protocols
/// - **Halo 2 proving system** replaces the previous trusted setup zk-SNARKs
///
/// Each sent output contains components of the Orchard note:
/// - Diversifier: Part of the recipient's shielded address derivation
/// - Public key: The recipient's public key for the transaction
/// - Value: The amount of ZEC transferred
/// - Rho/Psi: Protocol-specific randomness values
/// - Rcm: Random commitment material
///
/// # Data Preservation
/// During wallet migration, Orchard sent output information must be preserved to maintain
/// the ability to generate payment proofs for regulatory compliance, auditing,
/// or other selective disclosure purposes. The sending wallet is the only entity
/// that has this information in plaintext form.
///
/// # Examples
/// ```
/// # use zewif::{OrchardSentOutput, Blob, Amount};
/// # use anyhow::Result;
/// # fn example() -> Result<()> {
/// // Create a new Orchard sent output with all required components
/// let diversifier = Blob::<11>::default();
/// let recipient_pk = Blob::default();
/// let value = Amount::from_u64(10_000_000)?; // 0.1 ZEC
/// let rho = Blob::default();
/// let psi = Blob::default();
/// let rcm = Blob::default();
///
/// let sent_output = OrchardSentOutput::from_parts(
///     0,
///     "".to_string(),
///     diversifier,
///     recipient_pk,
///     value,
///     rho,
///     psi,
///     rcm,
///     None
/// );
///
/// // Check the value was set correctly
/// let value_zatoshi: i64 = sent_output.value().into();
/// assert_eq!(value_zatoshi, 10_000_000);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OrchardSentOutput {
    /// The index of the sent output in the transaction.
    index: usize,

    /// The string representation of the address to which the output was sent.
    ///
    /// This should be a Unified address with an Orchard receiver. We preserve the original address
    /// string because in the case of Unified addresses, it is not otherwise possible to
    /// reconstruct the data for receivers other than the Orchard receiver, and as a consequence
    /// the recipient address in a restored wallet would appear different than in the source
    /// wallet.
    recipient_address: String,

    /// The diversifier used in deriving the recipient's shielded address.
    ///
    /// This 11-byte value serves the same purpose as in Sapling, enabling address
    /// diversity for privacy. It is part of the note plaintext and essential for
    /// identifying the recipient during selective disclosure.
    diversifier: Blob<11>,

    /// The recipient's public key, serialized in compressed form.
    ///
    /// This 32-byte value represents a point on the Pallas curve, distinct from Sapling's
    /// Jubjub curve. It is included in the note plaintext and necessary for verifying
    /// the recipient in proofs or audits.
    receipient_public_key: Blob<32>,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer denotes the amount sent, with the same maximum value
    /// constraint as Sapling (2^63 - 1 zatoshis). It is a core component of the note
    /// for tracking and proving the transaction amount.
    value: Amount,

    /// A randomness element used in Orchard's note encryption and commitment.
    ///
    /// This 32-byte value (an element of the Pallas curve's field F_q) is unique to Orchard,
    /// enhancing privacy by contributing to the note's uniqueness. It is stored for
    /// reconstructing the note during selective disclosure.
    rho: Blob<32>,

    /// Another randomness element used in Orchard's note construction.
    ///
    /// This 32-byte value (also an element of F_q) further strengthens privacy in Orchard
    /// transactions. It is part of the note plaintext and required for generating proofs
    /// that validate the sent output.
    psi: Blob<32>,

    /// The random commitment material used in the note commitment.
    ///
    /// This 32-byte value (256-bit scalar) serves a similar role to Sapling's rcm, masking
    /// the note's contents on the blockchain. It is stored to enable the wallet to
    /// regenerate the commitment for proving payment details.
    rcm: Blob<32>,

    /// The memo attached to this output, if any.
    memo: Option<Memo>,
}

impl Indexed for OrchardSentOutput {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl OrchardSentOutput {
    /// Creates a new `OrchardSentOutput` from its constituent parts.
    ///
    /// This constructor creates an Orchard sent output with all required components
    /// for transaction reconstruction and selective disclosure.
    ///
    /// # Arguments
    /// * `diversifier` - The 11-byte diversifier used in deriving the recipient's address
    /// * `receipient_public_key` - The recipient's compressed public key (32 bytes)
    /// * `value` - The amount of ZEC sent
    /// * `rho` - The randomness element for note encryption (32 bytes)
    /// * `psi` - Additional randomness element for note construction (32 bytes)
    /// * `rcm` - Random commitment material (32 bytes)
    ///
    /// # Returns
    /// A new `OrchardSentOutput` instance with the provided values.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// # fn example() -> Result<()> {
    /// let diversifier = Blob::<11>::default();
    /// let recipient_pk = Blob::default();
    /// let value = Amount::from_u64(5_000_000)?; // 0.05 ZEC
    /// let rho = Blob::default();
    /// let psi = Blob::default();
    /// let rcm = Blob::default();
    ///
    /// let sent_output = OrchardSentOutput::from_parts(
    ///     0,
    ///     "".to_string(),
    ///     diversifier,
    ///     recipient_pk,
    ///     value,
    ///     rho,
    ///     psi,
    ///     rcm,
    ///     None
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        index: usize,
        recipient_address: String,
        diversifier: Blob<11>,
        receipient_public_key: Blob<32>,
        value: Amount,
        rho: Blob<32>,
        psi: Blob<32>,
        rcm: Blob<32>,
        memo: Option<Memo>,
    ) -> Self {
        Self {
            index,
            recipient_address,
            diversifier,
            receipient_public_key,
            value,
            rho,
            psi,
            rcm,
            memo,
        }
    }

    /// Returns the string representation of the address used in construction of the output.
    ///
    /// This will be a Unified address with an Orchard receiver. We preserve the original address
    /// string because in the case of Unified addresses, it is not otherwise possible to
    /// reconstruct the data for receivers other than the Orchard receiver, and as a consequence
    /// the recipient address in a restored wallet would appear different than in the source
    /// wallet.
    pub fn recipient_address(&self) -> &str {
        &self.recipient_address
    }

    /// Sets the string representation of the address used in construction of the output.
    pub fn set_recipient_address(&mut self, recipient_address: String) {
        self.recipient_address = recipient_address;
    }

    /// Returns a reference to the diversifier used in the recipient's address derivation.
    ///
    /// The diversifier is an 11-byte value that's part of Orchard shielded address construction.
    /// It allows multiple unique addresses to be generated from a single key pair, enhancing
    /// privacy by preventing address reuse.
    ///
    /// # Returns
    /// A reference to the 11-byte `Blob` containing the diversifier.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let diversifier = Blob::<11>::default();
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), diversifier.clone(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let div = sent_output.diversifier();
    /// assert_eq!(div, &diversifier);
    /// # Ok(())
    /// # }
    /// ```
    pub fn diversifier(&self) -> &Blob<11> {
        &self.diversifier
    }

    /// Returns a reference to the recipient's public key.
    ///
    /// This is a 32-byte representation of a point on the Pallas curve, used to encrypt
    /// the note content for the recipient. It is part of the plaintext information
    /// that the sender's wallet must store to enable selective disclosure.
    ///
    /// # Returns
    /// A reference to the recipient's public key as a `Blob<32>`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let pk = Blob::default();
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), pk.clone(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let recipient_pk = sent_output.receipient_public_key();
    /// # Ok(())
    /// # }
    /// ```
    pub fn receipient_public_key(&self) -> &Blob<32> {
        &self.receipient_public_key
    }

    /// Returns the value (amount) of ZEC sent in this output.
    ///
    /// This represents the amount of ZEC transferred in this specific note,
    /// measured in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// # Returns
    /// The amount of ZEC as an `Amount`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let value = Amount::from_u64(15_000_000)?;
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), value,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let amount = sent_output.value();
    /// let zats: i64 = amount.into();
    /// assert_eq!(zats, 15_000_000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn value(&self) -> Amount {
        self.value
    }

    /// Returns a reference to the `rho` randomness element.
    ///
    /// The `rho` value is a 32-byte element specific to Orchard that serves as
    /// randomness for note encryption and nullifier derivation. It is stored by
    /// the sender to enable selective disclosure of transaction details.
    ///
    /// # Returns
    /// A reference to the `rho` value as a `Blob<32>`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let rho = Blob::default();
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     rho.clone(), Blob::default(), Blob::default(), None);
    /// #
    /// let r = sent_output.rho();
    /// # Ok(())
    /// # }
    /// ```
    pub fn rho(&self) -> &Blob<32> {
        &self.rho
    }

    /// Returns a reference to the `psi` randomness element.
    ///
    /// The `psi` value is a 32-byte value specific to Orchard's protocol used for
    /// cryptographic operations within the note structure. It is not found in Sapling
    /// and is part of Orchard's enhanced privacy design.
    ///
    /// # Returns
    /// A reference to the `psi` value as a `Blob<32>`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let psi = Blob::default();
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), psi.clone(), Blob::default(), None);
    /// #
    /// let p = sent_output.psi();
    /// # Ok(())
    /// # }
    /// ```
    pub fn psi(&self) -> &Blob<32> {
        &self.psi
    }

    /// Returns a reference to the random commitment material.
    ///
    /// The rcm (random commitment material) is a 32-byte value used in constructing
    /// the note commitment on the blockchain. It ensures privacy by masking the
    /// note's contents. The sender must store this value to enable selective disclosure
    /// or payment proofs.
    ///
    /// # Returns
    /// A reference to the random commitment material as a `Blob<32>`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let rcm = Blob::default();
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), rcm.clone(), None);
    /// #
    /// let r = sent_output.rcm();
    /// # Ok(())
    /// # }
    /// ```
    pub fn rcm(&self) -> &Blob<32> {
        &self.rcm
    }

    /// Sets the diversifier for this sent output.
    ///
    /// # Arguments
    /// * `diversifier` - The 11-byte diversifier value used in the recipient's shielded address
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let new_diversifier = Blob::<11>::default();
    /// sent_output.set_diversifier(new_diversifier);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_diversifier(&mut self, diversifier: Blob<11>) {
        self.diversifier = diversifier;
    }

    /// Sets the recipient's public key.
    ///
    /// # Arguments
    /// * `receipient_public_key` - The 32-byte recipient public key value
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let pk = Blob::default();
    /// sent_output.set_receipient_public_key(pk);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_receipient_public_key(&mut self, receipient_public_key: Blob<32>) {
        self.receipient_public_key = receipient_public_key;
    }

    /// Sets the value (amount) of ZEC for this sent output.
    ///
    /// # Arguments
    /// * `value` - The amount of ZEC to set
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let amount = Amount::from_u64(25_000_000)?; // 0.25 ZEC
    /// sent_output.set_value(amount);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    /// Sets the `rho` randomness element for this sent output.
    ///
    /// # Arguments
    /// * `rho` - The 32-byte randomness element
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let rho = Blob::default();
    /// sent_output.set_rho(rho);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_rho(&mut self, rho: Blob<32>) {
        self.rho = rho;
    }

    /// Sets the `psi` randomness element for this sent output.
    ///
    /// # Arguments
    /// * `psi` - The 32-byte randomness element
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let psi = Blob::default();
    /// sent_output.set_psi(psi);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_psi(&mut self, psi: Blob<32>) {
        self.psi = psi;
    }

    /// Sets the random commitment material for this sent output.
    ///
    /// # Arguments
    /// * `rcm` - The 32-byte random commitment material
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Blob::<11>::default(), Blob::default(), Amount::from_u64(1000)?,
    /// #     Blob::default(), Blob::default(), Blob::default(), None);
    /// #
    /// let rcm = Blob::default();
    /// sent_output.set_rcm(rcm);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_rcm(&mut self, rcm: Blob<32>) {
        self.rcm = rcm;
    }

    /// Returns the memo associated with the output, if known.
    pub fn memo(&self) -> Option<&Memo> {
        self.memo.as_ref()
    }

    /// Sets the memo associated with the output.
    pub fn set_memo(&mut self, memo: Option<Memo>) {
        self.memo = memo;
    }
}

impl From<OrchardSentOutput> for Envelope {
    fn from(value: OrchardSentOutput) -> Self {
        Envelope::new(value.index)
            .add_type("OrchardSentOutput")
            .add_assertion("recipient_address", value.recipient_address)
            .add_assertion("diversifier", value.diversifier)
            .add_assertion("receipient_public_key", value.receipient_public_key)
            .add_assertion("value", value.value)
            .add_assertion("rho", value.rho)
            .add_assertion("psi", value.psi)
            .add_assertion("rcm", value.rcm)
            .add_optional_assertion("memo", value.memo)
    }
}

impl TryFrom<Envelope> for OrchardSentOutput {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope
            .check_type_envelope("OrchardSentOutput")
            .context("OrchardSentOutput")?;
        let index = envelope.extract_subject().context("index")?;
        let recipient_address = envelope
            .extract_object_for_predicate("recipient_address")
            .context("recipient_address")?;
        let diversifier = envelope
            .extract_object_for_predicate("diversifier")
            .context("diversifier")?;
        let receipient_public_key = envelope
            .extract_object_for_predicate("receipient_public_key")
            .context("receipient_public_key")?;
        let value = envelope
            .extract_object_for_predicate("value")
            .context("value")?;
        let rho = envelope
            .extract_object_for_predicate("rho")
            .context("rho")?;
        let psi = envelope
            .extract_object_for_predicate("psi")
            .context("psi")?;
        let rcm = envelope
            .extract_object_for_predicate("rcm")
            .context("rcm")?;
        let memo = envelope
            .extract_optional_object_for_predicate("memo")
            .context("memo")?;

        Ok(OrchardSentOutput {
            index,
            recipient_address,
            diversifier,
            receipient_public_key,
            value,
            rho,
            psi,
            rcm,
            memo,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amount, Blob, Memo, UnifiedAddress, test_envelope_roundtrip};

    use super::OrchardSentOutput;

    impl crate::RandomInstance for OrchardSentOutput {
        fn random() -> Self {
            Self {
                index: 0,
                recipient_address: UnifiedAddress::random().address().to_string(),
                diversifier: Blob::random(),
                receipient_public_key: Blob::random(),
                value: Amount::random(),
                rho: Blob::random(),
                psi: Blob::random(),
                rcm: Blob::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_envelope_roundtrip!(OrchardSentOutput);
}
