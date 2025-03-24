use super::{Amount, Blob, u256};

/// Represents a sent output in an Orchard shielded transaction within a Zcash wallet.
///
/// This structure stores the plaintext details of an Orchard note that was sent by the
/// wallet, which are not recoverable from the blockchain. It supports selective disclosure
/// for proving payments to shielded addresses, enhancing privacy and compliance features.
/// Introduced in Network Upgrade 5 (NU5), the Orchard protocol builds on Sapling with
/// additional privacy enhancements, using the Pallas curve and new cryptographic primitives.
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
    /// Creates a new Orchard sent output with the specified parameters.
    ///
    /// # Arguments
    ///
    /// * `diversifier` - The 11-byte diversifier used in deriving the recipient's address
    /// * `receipient_public_key` - The recipient's compressed public key (32 bytes)
    /// * `value` - The amount of ZEC sent in zatoshis
    /// * `rho` - The randomness element for note encryption (32 bytes)
    /// * `psi` - Additional randomness element for note construction (32 bytes)
    /// * `rcm` - Random commitment material (32 bytes)
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

    // Getters
    pub fn diversifier(&self) -> &Blob<11> {
        &self.diversifier
    }

    pub fn receipient_public_key(&self) -> &u256 {
        &self.receipient_public_key
    }

    pub fn value(&self) -> Amount {
        self.value
    }

    pub fn rho(&self) -> &u256 {
        &self.rho
    }

    pub fn psi(&self) -> &u256 {
        &self.psi
    }

    pub fn rcm(&self) -> &u256 {
        &self.rcm
    }

    // Setters
    pub fn set_diversifier(&mut self, diversifier: Blob<11>) {
        self.diversifier = diversifier;
    }

    pub fn set_receipient_public_key(&mut self, receipient_public_key: u256) {
        self.receipient_public_key = receipient_public_key;
    }

    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    pub fn set_rho(&mut self, rho: u256) {
        self.rho = rho;
    }

    pub fn set_psi(&mut self, psi: u256) {
        self.psi = psi;
    }

    pub fn set_rcm(&mut self, rcm: u256) {
        self.rcm = rcm;
    }
}
