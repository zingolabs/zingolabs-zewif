use std::{iter::Sum, ops::{Add, Mul, Neg, Sub}};

use anyhow::{Error, Result, anyhow, bail};

use super::parser::prelude::*;
use crate::parse;

pub const COIN: u64 = 1_0000_0000;
pub const MAX_MONEY: u64 = 21_000_000 * COIN;
pub const MAX_BALANCE: i64 = MAX_MONEY as i64;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(i64);

impl Parse for Amount {
    fn parse(p: &mut Parser) -> Result<Self> {
        let zat_balance = parse!(p, i64, "Zat balance")?;
        Amount::try_from(zat_balance).map_err(|_| anyhow!("Invalid Zat balance: {}", zat_balance))
    }
}

impl Amount {
    /// Returns a zero-valued Amount.
    pub const fn zero() -> Self {
        Amount(0)
    }

    /// Creates a constant Amount from an i64.
    ///
    /// Panics: if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub const fn const_from_i64(amount: i64) -> Self {
        assert!(-MAX_BALANCE <= amount && amount <= MAX_BALANCE); // contains is not const
        Amount(amount)
    }

    /// Creates a constant Amount from a u64.
    ///
    /// Panics: if the amount is outside the range `{0..MAX_BALANCE}`.
    pub const fn const_from_u64(amount: u64) -> Self {
        assert!(amount <= MAX_MONEY); // contains is not const
        Amount(amount as i64)
    }

    /// Creates an Amount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub fn from_i64(amount: i64) -> Result<Self> {
        if (-MAX_BALANCE..=MAX_BALANCE).contains(&amount) {
            Ok(Amount(amount))
        } else if amount < -MAX_BALANCE {
            bail!("Amount underflow: {}", amount)
        } else {
            bail!("Amount overflow: {}", amount)
        }
    }

    /// Creates a non-negative Amount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_nonnegative_i64(amount: i64) -> Result<Self> {
        if (0..=MAX_BALANCE).contains(&amount) {
            Ok(Amount(amount))
        } else if amount < 0 {
            bail!("Amount underflow: {}", amount)
        } else {
            bail!("Amount overflow: {}", amount)
        }
    }

    /// Creates an Amount from a u64.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_MONEY}`.
    pub fn from_u64(amount: u64) -> Result<Self> {
        if amount <= MAX_MONEY {
            Ok(Amount(amount as i64))
        } else {
            bail!("Amount overflow: {}", amount)
        }
    }

    /// Reads an Amount from a signed 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub fn from_i64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = i64::from_le_bytes(bytes);
        Amount::from_i64(amount)
    }

    /// Reads a non-negative Amount from a signed 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_nonnegative_i64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = i64::from_le_bytes(bytes);
        Amount::from_nonnegative_i64(amount)
    }

    /// Reads an Amount from an unsigned 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_u64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = u64::from_le_bytes(bytes);
        Amount::from_u64(amount)
    }

    /// Returns the Amount encoded as a signed 64-bit little-endian integer.
    pub fn to_i64_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Returns `true` if `self` is positive and `false` if the Amount is zero or
    /// negative.
    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Returns `true` if `self` is negative and `false` if the Amount is zero or
    /// positive.
    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    pub fn sum<I: IntoIterator<Item = Amount>>(values: I) -> Option<Amount> {
        let mut result = Amount::zero();
        for value in values {
            result = (result + value)?;
        }
        Some(result)
    }
}

impl TryFrom<i64> for Amount {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        Amount::from_i64(value)
    }
}

impl From<Amount> for i64 {
    fn from(amount: Amount) -> i64 {
        amount.0
    }
}

impl From<&Amount> for i64 {
    fn from(amount: &Amount) -> i64 {
        amount.0
    }
}

impl TryFrom<Amount> for u64 {
    type Error = Error;

    fn try_from(value: Amount) -> Result<Self, Self::Error> {
        value
            .0
            .try_into()
            .map_err(|_| anyhow!("Amount underflow: {}", value.0))
    }
}

impl Add<Amount> for Amount {
    type Output = Option<Amount>;

    fn add(self, rhs: Amount) -> Option<Amount> {
        Amount::from_i64(self.0 + rhs.0).ok()
    }
}

impl Add<Amount> for Option<Amount> {
    type Output = Self;

    fn add(self, rhs: Amount) -> Option<Amount> {
        self.and_then(|lhs| lhs + rhs)
    }
}

impl Sub<Amount> for Amount {
    type Output = Option<Amount>;

    fn sub(self, rhs: Amount) -> Option<Amount> {
        Amount::from_i64(self.0 - rhs.0).ok()
    }
}

impl Sub<Amount> for Option<Amount> {
    type Output = Self;

    fn sub(self, rhs: Amount) -> Option<Amount> {
        self.and_then(|lhs| lhs - rhs)
    }
}

impl Sum<Amount> for Option<Amount> {
    fn sum<I: Iterator<Item = Amount>>(mut iter: I) -> Self {
        iter.try_fold(Amount::zero(), |acc, a| acc + a)
    }
}

impl<'a> Sum<&'a Amount> for Option<Amount> {
    fn sum<I: Iterator<Item = &'a Amount>>(mut iter: I) -> Self {
        iter.try_fold(Amount::zero(), |acc, a| acc + *a)
    }
}

impl Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self {
        Amount(-self.0)
    }
}

impl Mul<usize> for Amount {
    type Output = Option<Amount>;

    fn mul(self, rhs: usize) -> Option<Amount> {
        let rhs: i64 = rhs.try_into().ok()?;
        self.0
            .checked_mul(rhs)
            .and_then(|i| Amount::try_from(i).ok())
    }
}
