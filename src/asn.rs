//! # Archive Serial Number
//!
//! Archive serial number is a way to identify a physical document by writing on it a unique number.
//! It allows to easily find the numeric version of any of your documents.

use serde::Deserialize;

#[derive(Debug, Copy, Clone, Deserialize)]
pub struct ASN(u64);

impl From<u64> for ASN {
    fn from(value: u64) -> Self {
        Self(value)
    }
}
impl From<ASN> for u64 {
    fn from(value: ASN) -> Self {
        value.0
    }
}
impl ToString for ASN {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
