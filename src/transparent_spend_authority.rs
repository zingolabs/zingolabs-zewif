use super::SpendingKey;

/// The authority to spend from a transparent address.
#[derive(Debug, Clone)]
pub enum TransparentSpendAuthority {
    SpendingKey(SpendingKey),
    Derived,
}
