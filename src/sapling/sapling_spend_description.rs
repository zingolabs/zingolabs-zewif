use super::super::{Amount, BlockHeight, GrothProof, u256};
use crate::{test_envelope_roundtrip, Indexed};
use anyhow::Context;
use bc_envelope::prelude::*;

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
/// # use zewif::{Amount, BlockHeight, GrothProof, u256, Indexed};
/// # use zewif::sapling::SaplingSpendDescription;
/// // Create a new spend description
/// let mut spend = SaplingSpendDescription::new();
///
/// // Set the position of this spend in the transaction
/// spend.set_index(0);
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
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SaplingSpendDescription {
    /// The position of this spend in the transaction's list of Sapling spends
    index: usize,
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

bc_envelope::impl_attachable!(SaplingSpendDescription);

impl Indexed for SaplingSpendDescription {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl SaplingSpendDescription {
    /// Creates a new empty SaplingSpendDescription.
    pub fn new() -> Self {
        Self {
            index: 0,
            value: None,
            anchor_height: None,
            nullifier: u256::default(),
            zkproof: GrothProof::default(),
            attachments: Attachments::new(),
        }
    }

    // Getters
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

impl From<SaplingSpendDescription> for Envelope {
    fn from(value: SaplingSpendDescription) -> Self {
        let e = Envelope::new(value.index)
            .add_type("SaplingSpendDescription")
            .add_optional_assertion("value", value.value)
            .add_optional_assertion("anchor_height", value.anchor_height)
            .add_assertion("nullifier", value.nullifier)
            .add_assertion("zkproof", value.zkproof);
        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for SaplingSpendDescription {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("SaplingSpendDescription").context("SaplingSpendDescription")?;
        let index = envelope.extract_subject().context("index")?;
        let value = envelope.extract_optional_object_for_predicate("value").context("value")?;
        let anchor_height = envelope.extract_optional_object_for_predicate("anchor_height").context("anchor_height")?;
        let nullifier = envelope.extract_object_for_predicate("nullifier").context("nullifier")?;
        let zkproof = envelope.try_object_for_predicate("zkproof").context("zkproof")?;
        let attachments = Attachments::try_from_envelope(&envelope)?;
        Ok(SaplingSpendDescription {
            index,
            value,
            anchor_height,
            nullifier,
            zkproof,
            attachments,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for SaplingSpendDescription {
    fn random() -> Self {
        Self {
            index: 0,
            value: Amount::opt_random(),
            anchor_height: BlockHeight::opt_random(),
            nullifier: u256::random(),
            zkproof: GrothProof::random(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(SaplingSpendDescription);
