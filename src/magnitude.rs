//! Seven-level magnitude vocabulary.
//!
//! [`Magnitude`] names ordered qualitative strength without carrying
//! component-domain payloads.

use nota_codec::NotaEnum;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Ordered qualitative magnitude, from minimum through maximum.
#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEnum,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
pub enum Magnitude {
    Minimum,
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Maximum,
}

impl Magnitude {
    /// PascalCase NOTA record-head identifier for this magnitude.
    pub const fn as_record_head(self) -> &'static str {
        match self {
            Self::Minimum => "Minimum",
            Self::VeryLow => "VeryLow",
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
            Self::VeryHigh => "VeryHigh",
            Self::Maximum => "Maximum",
        }
    }

    /// Parse a PascalCase NOTA record-head identifier back to a
    /// [`Magnitude`]. Returns [`None`] for any other identifier.
    pub fn from_record_head(name: &str) -> Option<Self> {
        match name {
            "Minimum" => Some(Self::Minimum),
            "VeryLow" => Some(Self::VeryLow),
            "Low" => Some(Self::Low),
            "Medium" => Some(Self::Medium),
            "High" => Some(Self::High),
            "VeryHigh" => Some(Self::VeryHigh),
            "Maximum" => Some(Self::Maximum),
            _ => None,
        }
    }
}
