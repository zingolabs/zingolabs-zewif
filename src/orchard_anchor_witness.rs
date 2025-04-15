use anyhow::Context;
use bc_envelope::prelude::*;

use crate::{Anchor, test_envelope_roundtrip};

use super::OrchardWitness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrchardAnchorWitness {
    anchor: Anchor,
    witness: OrchardWitness,
}

impl OrchardAnchorWitness {
    pub fn new(anchor: Anchor, witness: OrchardWitness) -> Self {
        Self { anchor, witness }
    }

    pub fn anchor(&self) -> &Anchor {
        &self.anchor
    }

    pub fn witness(&self) -> &OrchardWitness {
        &self.witness
    }
}

impl From<OrchardAnchorWitness> for Envelope {
    fn from(value: OrchardAnchorWitness) -> Self {
        Envelope::new(value.anchor)
            .add_type("OrchardAnchorWitness")
            .add_assertion("witness", value.witness)
    }
}

impl TryFrom<Envelope> for OrchardAnchorWitness {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("OrchardAnchorWitness").context("OrchardAnchorWitness")?;
        let anchor = envelope.extract_subject().context("anchor")?;
        let witness = envelope.try_object_for_predicate("witness").context("witness")?;
        Ok(OrchardAnchorWitness { anchor, witness })
    }
}

#[cfg(test)]
impl crate::RandomInstance for OrchardAnchorWitness {
    fn random() -> Self {
        let anchor = Anchor::random();
        let witness = OrchardWitness::random();
        Self::new(anchor, witness)
    }
}

test_envelope_roundtrip!(OrchardAnchorWitness);
