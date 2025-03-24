use std::ops::{Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use anyhow::{Context, Result, bail};

use super::parser::prelude::*;

/// A fixed-size byte array.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Blob<const N: usize>([u8; N]);

impl<const N: usize> Blob<N> {
    pub fn new(data: [u8; N]) -> Self {
        Self(data)
    }

    pub fn len(&self) -> usize {
        N
    }

    pub fn is_empty(&self) -> bool {
        N == 0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn from_slice(data: &[u8]) -> Result<Self> {
        if data.len() != N {
            bail!("Invalid data length: expected {}, got {}", N, data.len());
        }
        let mut bytes = [0u8; N];
        bytes.copy_from_slice(data);
        Ok(Self::new(bytes))
    }

    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Self::from_slice(&data)
    }

    pub fn from_hex(hex: &str) -> Self {
        let data = hex::decode(hex).expect("Decoding hex string");
        Self::from_vec(data).expect("Creating Blob from hex")
    }

    pub fn reverse(&mut self) {
        self.0.reverse();
    }
}

impl<const N: usize> Default for Blob<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> Index<usize> for Blob<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Blob<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Index<Range<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeTo<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeFrom<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeFull> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeInclusive<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeToInclusive<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> From<Blob<N>> for [u8; N] {
    fn from(blob: Blob<N>) -> Self {
        blob.0
    }
}

impl<const N: usize> AsRef<[u8]> for Blob<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> std::fmt::Debug for Blob<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Blob<{}>({})", N, hex::encode(self.0))
    }
}

impl<const N: usize> From<Blob<N>> for Vec<u8> {
    fn from(blob: Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<&Blob<N>> for Vec<u8> {
    fn from(blob: &Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<Vec<u8>> for Blob<N> {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data).unwrap()
    }
}

impl<const N: usize> From<&[u8]> for Blob<N> {
    fn from(data: &[u8]) -> Self {
        Self::from_vec(data.to_vec()).unwrap()
    }
}

impl<const N: usize> Parse for Blob<N> {
    fn parse(parser: &mut Parser) -> Result<Self>
    where
        Self: Sized,
    {
        let data = parser
            .next(N)
            .with_context(|| format!("Parsing Blob<{}>", N))?;
        Self::from_slice(data)
    }
}

pub type Blob20 = Blob<20>;
pub type Blob32 = Blob<32>;
pub type Blob64 = Blob<64>;
