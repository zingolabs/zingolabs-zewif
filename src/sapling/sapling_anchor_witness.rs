use anyhow::Context;
use bc_envelope::prelude::*;

use crate::{test_envelope_roundtrip, Anchor};

use super::SaplingWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaplingAnchorWitness {
    anchor: Anchor,
    witness: SaplingWitness,
}

impl SaplingAnchorWitness {
    pub fn new(anchor: Anchor, witness: SaplingWitness) -> Self {
        Self { anchor, witness }
    }

    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn witness(&self) -> &SaplingWitness {
        &self.witness
    }
}

impl From<SaplingAnchorWitness> for Envelope {
    fn from(value: SaplingAnchorWitness) -> Self {
        Envelope::new(value.anchor)
            .add_type("SaplingAnchorWitness")
            .add_assertion("witness", value.witness)
    }
}

impl TryFrom<Envelope> for SaplingAnchorWitness {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("SaplingAnchorWitness").context("SaplingAnchorWitness")?;
        let anchor = envelope.extract_subject().context("anchor")?;
        let witness = envelope.try_object_for_predicate("witness").context("witness")?;
        Ok(SaplingAnchorWitness { anchor, witness })
    }
}

#[cfg(test)]
impl crate::RandomInstance for SaplingAnchorWitness {
    fn random() -> Self {
        let anchor = Anchor::random();
        let witness = SaplingWitness::random();
        Self::new(anchor, witness)
    }
}

test_envelope_roundtrip!(SaplingAnchorWitness);
