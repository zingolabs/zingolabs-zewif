use super::super::{u256, Amount, Blob};

/// Represents a sent output in a Sapling shielded transaction within a Zcash wallet.
///
/// This structure stores the plaintext details of a Sapling note that was sent by the wallet,
/// which are not recoverable from the blockchain. It is used for selective disclosure,
/// allowing the sender to prove a payment was made to a specific shielded address without
/// revealing the full transaction details. This is particularly useful for auditing or
/// compliance with regulations. The Sapling protocol, activated in June 2018, enhances
/// privacy using zk-SNARKs over the earlier Sprout protocol.
#[derive(Debug, Clone)]
pub struct SaplingSentOutput {
    /// The diversifier used in deriving the recipient's shielded address.
    ///
    /// This 11-byte value is part of the Sapling address construction, allowing multiple
    /// unique addresses to be generated from a single key pair. It is critical for
    /// identifying the recipient and reconstructing the note for proof generation.
    diversifier: Blob<11>,

    /// The recipient's public key, serialized in compressed form.
    ///
    /// This 32-byte value represents a point on the Jubjub curve, used in Sapling's
    /// cryptographic operations. It is part of the note plaintext and is needed to
    /// verify the recipient of the sent funds during selective disclosure.
    receipient_public_key: u256,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer specifies the amount transferred. It is constrained
    /// by the protocol to a maximum value (2^63 - 1 zatoshis), ensuring it fits within
    /// the note's value field for Sapling transactions.
    value: Amount,

    /// The random commitment material used in the note commitment.
    ///
    /// This 32-byte value (256-bit scalar) is a randomly generated element used to
    /// construct the note commitment on the blockchain, ensuring privacy by masking
    /// the note's contents. It is stored here to allow reconstruction of the commitment
    /// for proving purposes.
    rcm: u256,
}
impl SaplingSentOutput {
    /// Creates a new empty SaplingSentOutput instance with default values.
    pub fn new() -> Self {
        Self {
            diversifier: Blob::default(),
            receipient_public_key: u256::default(),
            value: Amount::zero(),
            rcm: u256::default(),
        }
    }

    /// Returns a reference to the diversifier.
    pub fn diversifier(&self) -> &Blob<11> {
        &self.diversifier
    }

    /// Sets the diversifier.
    pub fn set_diversifier(&mut self, diversifier: Blob<11>) {
        self.diversifier = diversifier;
    }

    /// Returns a reference to the recipient's public key.
    pub fn receipient_public_key(&self) -> &u256 {
        &self.receipient_public_key
    }

    /// Sets the recipient's public key.
    pub fn set_receipient_public_key(&mut self, key: u256) {
        self.receipient_public_key = key;
    }

    /// Returns the value.
    pub fn value(&self) -> Amount {
        self.value
    }

    /// Sets the value.
    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    /// Returns a reference to the random commitment material.
    pub fn rcm(&self) -> &u256 {
        &self.rcm
    }

    /// Sets the random commitment material.
    pub fn set_rcm(&mut self, rcm: u256) {
        self.rcm = rcm;
    }
}

impl Default for SaplingSentOutput {
    fn default() -> Self {
        Self::new()
    }
}
