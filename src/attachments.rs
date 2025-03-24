use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

#[derive(Debug, Clone)]
pub struct Attachments {
    envelopes: HashMap<Digest, Envelope>,
}

impl Default for Attachments {
    fn default() -> Self {
        Self::new()
    }
}

impl Attachments {
    pub fn new() -> Self {
        Self { envelopes: HashMap::new() }
    }

    pub fn add(
        &mut self,
        payload: impl EnvelopeEncodable,
        vendor: &str,
        conforms_to: Option<&str>,
    ) {
        let attachment = Envelope::new_attachment(payload, vendor, conforms_to);
        self.envelopes
            .insert(attachment.digest().into_owned(), attachment);
    }

    pub fn get(&self, digest: &Digest) -> Option<&Envelope> {
        self.envelopes.get(digest)
    }

    pub fn remove(&mut self, digest: &Digest) -> Option<Envelope> {
        self.envelopes.remove(digest)
    }

    pub fn clear(&mut self) {
        self.envelopes.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.envelopes.is_empty()
    }
}

#[allow(dead_code)]
pub trait Attachable {
    fn attachments(&self) -> &Attachments;
    fn attachments_mut(&mut self) -> &mut Attachments;

    fn add_attachment(
        &mut self,
        payload: impl EnvelopeEncodable,
        vendor: &str,
        conforms_to: Option<&str>,
    ) {
        self.attachments_mut().add(payload, vendor, conforms_to);
    }

    fn get_attachment(&self, digest: &Digest) -> Option<&Envelope> {
        self.attachments().get(digest)
    }

    fn remove_attachment(&mut self, digest: &Digest) -> Option<Envelope> {
        self.attachments_mut().remove(digest)
    }

    fn clear_attachments(&mut self) {
        self.attachments_mut().clear();
    }

    fn has_attachments(&self) -> bool {
        !self.attachments().is_empty()
    }
}
