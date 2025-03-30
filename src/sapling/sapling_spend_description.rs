use crate::impl_attachable;
use super::super::{Amount, Attachments, BlockHeight, GrothProof, u256};

/// A description of a spent Sapling note in a shielded transaction.
///
/// `SaplingSpendDescription` represents the spending of a Sapling shielded note in a Zcash
/// transaction. It contains cryptographic proofs and metadata that verify the spender's
/// authority and prevent double-spending, without revealing the note's details.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling shielded protocol:
///
/// - **Notes**: Encrypted value containers in the shielded pool that can only be spent by
///   their owners
///
/// - **Nullifiers**: Unique identifiers that prevent double-spending of notes. When a note is
///   spent, its nullifier is published on the blockchain, making any future attempts to spend
///   the same note invalid.
///
/// - **Zero-Knowledge Proofs**: Cryptographic proofs (Groth16) that verify the spender owns
///   the note and that the transaction is balanced, without revealing any sensitive details
///
/// - **Anchors**: Commitments to the state of the note commitment tree at a specific block
///   height, proving the note existed at that time
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **Note Value**: The amount being spent, if known to the wallet
/// - **Nullifier**: The unique identifier that prevents double-spending this note
/// - **Anchor Height**: The block height at which the note commitment tree was anchored
/// - **ZK Proof**: The cryptographic proof that validates the spend
/// - **Spend Index**: The position of this spend within the transaction's Sapling spends
///
/// # Examples
/// ```
/// use zewif::{Amount, BlockHeight, GrothProof, u256};
/// use zewif::sapling::SaplingSpendDescription;
///
/// // Create a new spend description
/// let mut spend = SaplingSpendDescription::new();
///
/// // Set the position of this spend in the transaction
/// spend.set_spend_index(0);
///
/// // Set the note value (if known)
/// let value = Amount::from_u64(100_000_000).unwrap(); // 1 ZEC
/// spend.set_value(Some(value));
///
/// // Set the anchor block height
/// let height = BlockHeight::from(1_000_000);
/// spend.set_anchor_height(Some(height));
///
/// // Set the nullifier (prevents double-spending)
/// let nullifier = u256::default();
/// spend.set_nullifier(nullifier);
///
/// // A real implementation would set a valid zero-knowledge proof
/// let zkproof = GrothProof::default();
/// spend.set_zkproof(zkproof);
///
/// // The nullifier will be published on the blockchain
/// assert_eq!(spend.nullifier(), &nullifier);
/// ```
#[derive(Debug, Clone, Default)]
pub struct SaplingSpendDescription {
    /// The position of this spend in the transaction's list of Sapling spends
    spend_index: u32,
    /// The value of the input note being spent, if known to the wallet
    value: Option<Amount>,
    /// The block height that the note commitment tree anchor corresponds to
    anchor_height: Option<BlockHeight>,
    /// A unique identifier that prevents double-spending of this note
    nullifier: u256,
    /// A zero-knowledge proof verifying the spender's authority and transaction validity
    zkproof: GrothProof,
    /// Additional metadata attachments for this spend
    attachments: Attachments,
}

impl_attachable!(SaplingSpendDescription);

impl SaplingSpendDescription {
    /// Creates a new empty SaplingSpendDescription.
    pub fn new() -> Self {
        Self {
            spend_index: 0,
            value: None,
            anchor_height: None,
            nullifier: u256::default(),
            zkproof: GrothProof::default(),
            attachments: Attachments::new(),
        }
    }

    // Getters
    pub fn spend_index(&self) -> u32 {
        self.spend_index
    }

    pub fn value(&self) -> Option<Amount> {
        self.value
    }

    pub fn anchor_height(&self) -> Option<BlockHeight> {
        self.anchor_height
    }

    pub fn nullifier(&self) -> &u256 {
        &self.nullifier
    }

    pub fn zkproof(&self) -> &GrothProof {
        &self.zkproof
    }

    pub fn attachments(&self) -> &Attachments {
        &self.attachments
    }

    // Setters
    pub fn set_spend_index(&mut self, spend_index: u32) -> &mut Self {
        self.spend_index = spend_index;
        self
    }

    pub fn set_value(&mut self, value: Option<Amount>) -> &mut Self {
        self.value = value;
        self
    }

    pub fn set_anchor_height(&mut self, anchor_height: Option<BlockHeight>) -> &mut Self {
        self.anchor_height = anchor_height;
        self
    }

    pub fn set_nullifier(&mut self, nullifier: u256) -> &mut Self {
        self.nullifier = nullifier;
        self
    }

    pub fn set_zkproof(&mut self, zkproof: GrothProof) -> &mut Self {
        self.zkproof = zkproof;
        self
    }
}
