use anyhow::Result;

use super::Data;
use crate::{parse, parser::prelude::*};
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Script(Data);

impl Script {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Parse for Script {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "Script")?))
    }
}

impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Script<{}>({})", self.0.len(), hex::encode(self))
    }
}

impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Script> for Data {
    fn from(script: Script) -> Self {
        script.0
    }
}

impl Index<usize> for Script {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Script {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Range<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Range<usize>> for Script {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeTo<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeTo<usize>> for Script {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFrom<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFrom<usize>> for Script {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFull> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFull> for Script {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeToInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeToInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
