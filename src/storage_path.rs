//! # Storage path
//!
//! This part of the library is not implemented

use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct Id(u64);

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<Id> for u64 {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ToString for Id {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
