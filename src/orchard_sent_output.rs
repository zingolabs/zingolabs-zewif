use super::{Amount, Blob, u256};

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
/// use zewif::{OrchardSentOutput, Blob, u256, Amount};
/// use anyhow::Result;
///
/// # fn example() -> Result<()> {
/// // Create a new Orchard sent output with all required components
/// let diversifier = Blob::<11>::default();
/// let recipient_pk = u256::default();
/// let value = Amount::from_u64(10_000_000)?; // 0.1 ZEC
/// let rho = u256::default();
/// let psi = u256::default();
/// let rcm = u256::default();
///
/// let sent_output = OrchardSentOutput::new(
///     diversifier,
///     recipient_pk,
///     value,
///     rho,
///     psi,
///     rcm
/// );
///
/// // Check the value was set correctly
/// let value_zatoshi: i64 = sent_output.value().into();
/// assert_eq!(value_zatoshi, 10_000_000);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct OrchardSentOutput {
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
    receipient_public_key: u256,

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
    rho: u256,

    /// Another randomness element used in Orchard's note construction.
    ///
    /// This 32-byte value (also an element of F_q) further strengthens privacy in Orchard
    /// transactions. It is part of the note plaintext and required for generating proofs
    /// that validate the sent output.
    psi: u256,

    /// The random commitment material used in the note commitment.
    ///
    /// This 32-byte value (256-bit scalar) serves a similar role to Sapling's rcm, masking
    /// the note's contents on the blockchain. It is stored to enable the wallet to
    /// regenerate the commitment for proving payment details.
    rcm: u256,
}

impl OrchardSentOutput {
    /// Creates a new `OrchardSentOutput` with the specified parameters.
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
    /// use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// use anyhow::Result;
    ///
    /// # fn example() -> Result<()> {
    /// let diversifier = Blob::<11>::default();
    /// let recipient_pk = u256::default();
    /// let value = Amount::from_u64(5_000_000)?; // 0.05 ZEC
    /// let rho = u256::default();
    /// let psi = u256::default();
    /// let rcm = u256::default();
    ///
    /// let sent_output = OrchardSentOutput::new(
    ///     diversifier,
    ///     recipient_pk,
    ///     value,
    ///     rho,
    ///     psi,
    ///     rcm
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(
        diversifier: Blob<11>,
        receipient_public_key: u256,
        value: Amount,
        rho: u256,
        psi: u256,
        rcm: u256,
    ) -> Self {
        Self {
            diversifier,
            receipient_public_key,
            value,
            rho,
            psi,
            rcm,
        }
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
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let diversifier = Blob::<11>::default();
    /// # let sent_output = OrchardSentOutput::new(
    /// #     diversifier.clone(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
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
    /// A reference to the recipient's public key as a `u256`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let pk = u256::default();
    /// # let sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), pk.clone(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
    /// #
    /// let recipient_pk = sent_output.receipient_public_key();
    /// # Ok(())
    /// # }
    /// ```
    pub fn receipient_public_key(&self) -> &u256 {
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
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let value = Amount::from_u64(15_000_000)?;
    /// # let sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), value,
    /// #     u256::default(), u256::default(), u256::default());
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
    /// A reference to the `rho` value as a `u256`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let rho = u256::default();
    /// # let sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     rho.clone(), u256::default(), u256::default());
    /// #
    /// let r = sent_output.rho();
    /// # Ok(())
    /// # }
    /// ```
    pub fn rho(&self) -> &u256 {
        &self.rho
    }

    /// Returns a reference to the `psi` randomness element.
    ///
    /// The `psi` value is a 32-byte value specific to Orchard's protocol used for
    /// cryptographic operations within the note structure. It is not found in Sapling
    /// and is part of Orchard's enhanced privacy design.
    ///
    /// # Returns
    /// A reference to the `psi` value as a `u256`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let psi = u256::default();
    /// # let sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), psi.clone(), u256::default());
    /// #
    /// let p = sent_output.psi();
    /// # Ok(())
    /// # }
    /// ```
    pub fn psi(&self) -> &u256 {
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
    /// A reference to the random commitment material as a `u256`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let rcm = u256::default();
    /// # let sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), rcm.clone());
    /// #
    /// let r = sent_output.rcm();
    /// # Ok(())
    /// # }
    /// ```
    pub fn rcm(&self) -> &u256 {
        &self.rcm
    }

    /// Sets the diversifier for this sent output.
    ///
    /// # Arguments
    /// * `diversifier` - The 11-byte diversifier value used in the recipient's shielded address
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
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
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
    /// #
    /// let pk = u256::default();
    /// sent_output.set_receipient_public_key(pk);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_receipient_public_key(&mut self, receipient_public_key: u256) {
        self.receipient_public_key = receipient_public_key;
    }

    /// Sets the value (amount) of ZEC for this sent output.
    ///
    /// # Arguments
    /// * `value` - The amount of ZEC to set
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
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
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
    /// #
    /// let rho = u256::default();
    /// sent_output.set_rho(rho);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_rho(&mut self, rho: u256) {
        self.rho = rho;
    }

    /// Sets the `psi` randomness element for this sent output.
    ///
    /// # Arguments
    /// * `psi` - The 32-byte randomness element
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
    /// #
    /// let psi = u256::default();
    /// sent_output.set_psi(psi);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_psi(&mut self, psi: u256) {
        self.psi = psi;
    }

    /// Sets the random commitment material for this sent output.
    ///
    /// # Arguments
    /// * `rcm` - The 32-byte random commitment material
    ///
    /// # Examples
    /// ```
    /// # use zewif::{OrchardSentOutput, Blob, u256, Amount};
    /// # use anyhow::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::new(
    /// #     Blob::<11>::default(), u256::default(), Amount::from_u64(1000)?,
    /// #     u256::default(), u256::default(), u256::default());
    /// #
    /// let rcm = u256::default();
    /// sent_output.set_rcm(rcm);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_rcm(&mut self, rcm: u256) {
        self.rcm = rcm;
    }
}
