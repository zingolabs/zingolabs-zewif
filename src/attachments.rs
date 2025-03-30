//! Extensible metadata attachment system for ZeWIF wallet components.
//!
//! This module provides the infrastructure for attaching arbitrary metadata
//! to wallet objects. Attachments enable flexible, extensible data storage
//! without modifying the core data model, facilitating interoperability and
//! future compatibility.

use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

/// A container for vendor-specific metadata attachments in the ZeWIF format.
///
/// `Attachments` provides a flexible mechanism for attaching arbitrary metadata
/// to wallet objects without modifying their core structure. This enables various
/// wallet implementations to extend the data model with vendor-specific information
/// while maintaining interoperability.
///
/// # Zcash Concept Relation
/// Zcash wallet implementations often need to store additional metadata beyond
/// the core cryptographic material and transaction data. For example:
///
/// - Custom labels and tags for addresses and transactions
/// - Usage tracking and analytics data
/// - Implementation-specific configuration and preferences
/// - Additional proofs or verification data
///
/// The `Attachments` system allows this data to be preserved during wallet migration
/// without requiring standardization of every possible metadata field.
///
/// # Data Preservation
/// During wallet migration, attachments enable specialized data to be preserved
/// even when the receiving wallet implementation doesn't understand its structure.
/// The vendor and conformance information allows implementations to selectively
/// process attachments they recognize.
///
/// # Examples
/// ```ignore
/// use zewif::Attachments;
/// use bc_components::Digest;
///
/// // Create a new attachments container
/// let mut attachments = Attachments::new();
///
/// // Add a vendor-specific attachment
/// attachments.add("Transaction label: Coffee shop", "vendor.example", Some("labels.v1"));
///
/// // Attachments are indexed by their digest
/// // In a real implementation, you'd keep track of the digest when adding
/// let digest = Digest::from_bytes(b"digest"); // Example: creating a digest
/// if let Some(envelope) = attachments.get(&digest) {
///     // Process the attachment if found
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Attachments {
    /// Storage mapping from digest to envelope
    envelopes: HashMap<Digest, Envelope>,
}

impl Default for Attachments {
    fn default() -> Self {
        Self::new()
    }
}

impl Attachments {
    /// Creates a new empty attachments container.
    pub fn new() -> Self {
        Self { envelopes: HashMap::new() }
    }

    /// Adds a new attachment with the specified payload and metadata.
    ///
    /// # Arguments
    /// * `payload` - The data to attach, which must be encodable in an envelope
    /// * `vendor` - A string identifying the entity that defined the attachment format
    /// * `conforms_to` - An optional string identifying the structure the payload conforms to
    ///
    /// # Examples
    /// ```ignore
    /// # use zewif::Attachments;
    /// #
    /// let mut attachments = Attachments::new();
    ///
    /// // Add a simple string payload
    /// attachments.add("User label: Savings", "wallet.example.com", Some("labels.v1"));
    ///
    /// // Add a structured payload (in a real implementation, use an encodable type)
    /// let metadata = "metadata";
    /// attachments.add(metadata, "vendor.example", Some("metadata.v2"));
    /// ```
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

    /// Retrieves an attachment by its digest.
    ///
    /// # Arguments
    /// * `digest` - The unique digest of the attachment to retrieve
    ///
    /// # Returns
    /// A reference to the envelope if found, or None if no attachment exists with the given digest
    pub fn get(&self, digest: &Digest) -> Option<&Envelope> {
        self.envelopes.get(digest)
    }

    /// Removes an attachment by its digest.
    ///
    /// # Arguments
    /// * `digest` - The unique digest of the attachment to remove
    ///
    /// # Returns
    /// The removed envelope if found, or None if no attachment exists with the given digest
    pub fn remove(&mut self, digest: &Digest) -> Option<Envelope> {
        self.envelopes.remove(digest)
    }

    /// Removes all attachments from the container.
    pub fn clear(&mut self) {
        self.envelopes.clear();
    }

    /// Returns whether the container has any attachments.
    ///
    /// # Returns
    /// `true` if there are no attachments, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.envelopes.is_empty()
    }
}

/// A trait for types that can have metadata attachments.
///
/// `Attachable` provides a consistent interface for working with metadata attachments
/// across different wallet types. Types implementing this trait can store and retrieve
/// vendor-specific data without modifying their core structure.
///
/// # Zcash Concept Relation
/// Many components of a Zcash wallet benefit from the ability to store additional metadata:
///
/// - **Addresses**: Labels, usage history, contact information
/// - **Transactions**: Notes, categories, receipts
/// - **Accounts**: Names, preferences, policies
/// - **Keys**: Key origin information, backup status
///
/// # Examples
/// ```no_run
/// # use zewif::{Attachable, Attachments};
/// # use bc_components::Digest;
/// #
/// struct Transaction {
///     // Core transaction fields...
///     attachments: Attachments,
/// }
///
/// impl Attachable for Transaction {
///     fn attachments(&self) -> &Attachments {
///         &self.attachments
///     }
///
///     fn attachments_mut(&mut self) -> &mut Attachments {
///         &mut self.attachments
///     }
/// }
///
/// // Using the Attachable trait methods
/// let mut tx = Transaction { attachments: Attachments::new() };
/// tx.add_attachment("Payment for services", "example.com", Some("memo.v1"));
///
/// assert!(tx.has_attachments());
/// ```
#[allow(dead_code)]
pub trait Attachable {
    /// Returns a reference to the attachments container.
    fn attachments(&self) -> &Attachments;
    
    /// Returns a mutable reference to the attachments container.
    fn attachments_mut(&mut self) -> &mut Attachments;

    /// Adds a new attachment with the specified payload and metadata.
    ///
    /// # Arguments
    /// * `payload` - The data to attach, which must be encodable in an envelope
    /// * `vendor` - A string identifying the entity that defined the attachment format
    /// * `conforms_to` - An optional string identifying the structure the payload conforms to
    fn add_attachment(
        &mut self,
        payload: impl EnvelopeEncodable,
        vendor: &str,
        conforms_to: Option<&str>,
    ) {
        self.attachments_mut().add(payload, vendor, conforms_to);
    }

    /// Retrieves an attachment by its digest.
    ///
    /// # Arguments
    /// * `digest` - The unique digest of the attachment to retrieve
    ///
    /// # Returns
    /// A reference to the envelope if found, or None if no attachment exists with the given digest
    fn get_attachment(&self, digest: &Digest) -> Option<&Envelope> {
        self.attachments().get(digest)
    }

    /// Removes an attachment by its digest.
    ///
    /// # Arguments
    /// * `digest` - The unique digest of the attachment to remove
    ///
    /// # Returns
    /// The removed envelope if found, or None if no attachment exists with the given digest
    fn remove_attachment(&mut self, digest: &Digest) -> Option<Envelope> {
        self.attachments_mut().remove(digest)
    }

    /// Removes all attachments from the object.
    fn clear_attachments(&mut self) {
        self.attachments_mut().clear();
    }

    /// Returns whether the object has any attachments.
    ///
    /// # Returns
    /// `true` if there are attachments, `false` otherwise
    fn has_attachments(&self) -> bool {
        !self.attachments().is_empty()
    }
}
