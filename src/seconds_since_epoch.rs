use crate::{parse, parser::prelude::*};

use anyhow::Result;
use chrono::{SecondsFormat, TimeZone, Utc};

/// Represents a number of seconds since the Unix epoch.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecondsSinceEpoch(u64);

impl SecondsSinceEpoch {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl From<u64> for SecondsSinceEpoch {
    fn from(seconds: u64) -> Self {
        Self(seconds)
    }
}

impl From<SecondsSinceEpoch> for u64 {
    fn from(seconds: SecondsSinceEpoch) -> Self {
        seconds.0
    }
}

impl From<u32> for SecondsSinceEpoch {
    fn from(seconds: u32) -> Self {
        Self(seconds as u64)
    }
}

impl Parse for SecondsSinceEpoch {
    fn parse(p: &mut Parser) -> Result<Self> {
        let seconds = parse!(p, u64, "seconds")?;
        Ok(SecondsSinceEpoch(seconds))
    }
}

// Format as ISO-8601 date-time, e.g.: "2024-11-27T09:39:36Z"
impl std::fmt::Debug for SecondsSinceEpoch {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let dt = Utc
            .timestamp_opt(self.0 as i64, 0)
            .single()
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Secs, true);
        write!(f, "{}", dt)
    }
}
