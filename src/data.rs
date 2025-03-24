use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

use anyhow::{Context, Result};

use crate::{parse, parser::prelude::*};

use super::CompactSize;

#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
/// A variable-size byte array.
pub struct Data(Vec<u8>);

impl Data {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_bytes(data: impl AsRef<[u8]>) -> Self {
        Self(data.as_ref().to_vec())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_slice(data: &[u8]) -> Self {
        Self(data.to_vec())
    }

    pub fn from_vec(data: Vec<u8>) -> Self {
        Self(data)
    }

    pub fn from_hex(hex: &str) -> Result<Self> {
        Ok(Self(hex::decode(hex)?))
    }

    pub fn concat(a: &[&dyn AsRef<[u8]>]) -> Self {
        let mut bytes = Vec::new();
        for data in a {
            bytes.extend_from_slice(data.as_ref());
        }
        Self(bytes)
    }
}

impl Data {
    pub fn parse_len(parser: &mut Parser, len: usize) -> Result<Self> {
        let bytes = parser.next(len).context("Parsing Data")?;
        Ok(Self::from_slice(bytes))
    }
}

impl Parse for Data {
    fn parse(p: &mut Parser) -> Result<Self> {
        let len = parse!(p, CompactSize, "Data length")?;
        Self::parse_len(p, *len)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Data {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Range<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<Range<usize>> for Data {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeTo<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeTo<usize>> for Data {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeFrom<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeFrom<usize>> for Data {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeFull> for Data {
    type Output = [u8];

    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeFull> for Data {
    fn index_mut(&mut self, range: RangeFull) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeInclusive<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeInclusive<usize>> for Data {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeToInclusive<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeToInclusive<usize>> for Data {
    fn index_mut(&mut self, range: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data<{}>({})", self.len(), hex::encode(self))
    }
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        self
    }
}

impl From<Data> for Vec<u8> {
    fn from(data: Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<&Data> for Vec<u8> {
    fn from(data: &Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data)
    }
}

impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Self {
        Self::from_slice(data)
    }
}
