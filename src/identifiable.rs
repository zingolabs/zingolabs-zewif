#![allow(dead_code)]

use bc_components::ARID;

/// A trait for objects that have a unique identifier within the wallet interchange format.
///
/// `Identifiable` provides a standard way for significant wallet components to identify
/// themselves with a unique, cryptographically secure identifier. This facilitates
/// reference tracking, indexing, and cross-referencing between components during wallet
/// migration.
///
/// # Zcash Concept Relation
/// In wallets containing multiple accounts, numerous transactions, and various
/// private keys and addresses, maintaining precise references between related components
/// is essential. The `Identifiable` trait enables consistent tracking of:
///
/// - Wallet components across different implementations
/// - Relationships between accounts, transactions, and keys
/// - Hierarchical structures within the wallet
///
/// The ARID (Apparently Random IDentifier) format is especially useful as it's
/// collision-resistant and doesn't reveal any information about the identified objects.
///
/// # Data Preservation
/// During wallet migration, these identifiers ensure that relationships between
/// components are preserved, even when the internal representation might differ
/// between wallet implementations.
///
/// # Examples
/// ```no_run
/// # use zewif::Identifiable;
/// # use bc_components::ARID;
/// #
/// struct Account {
///     name: String,
///     arid: ARID,
///     // Other fields...
/// }
///
/// impl Identifiable for Account {
///     fn id(&self) -> ARID {
///         self.arid
///     }
/// }
///
/// // Later, the identifier can be used to find related components
/// fn find_account_transactions<'a>(account_id: &ARID, txs: &'a [Transaction]) -> Vec<&'a Transaction> {
///     txs.iter().filter(|tx| tx.account_id() == account_id).collect()
/// }
/// # struct Transaction { account_id: ARID }
/// # impl Transaction { fn account_id(&self) -> &ARID { &self.account_id } }
/// ```
pub trait Identifiable {
    /// Returns the unique identifier for this object.
    ///
    /// The identifier should be stable across serialization/deserialization cycles
    /// and should uniquely identify the object within its domain (e.g., all accounts,
    /// all transactions, etc.).
    fn id(&self) -> ARID;
}
