//! Eight-level magnitude vocabulary.
//!
//! [`Magnitude`] names ordered qualitative strength without carrying
//! component-domain payloads.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};

/// Ordered qualitative magnitude, from zero through maximum.
#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "nota-text",
    derive(nota::NotaEncode, nota::NotaDecode)
)]
#[repr(u8)]
pub enum Magnitude {
    Minimum = 0,
    VeryLow = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    VeryHigh = 5,
    Maximum = 6,
    /// Neutral bottom rung. Kept physically last so persisted rkyv
    /// discriminants for the original seven variants stay stable.
    Zero = 7,
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
            Self::Zero => "Zero",
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
            "Zero" => Some(Self::Zero),
            _ => None,
        }
    }

    const fn order_rank(self) -> u8 {
        match self {
            Self::Zero => 0,
            Self::Minimum => 1,
            Self::VeryLow => 2,
            Self::Low => 3,
            Self::Medium => 4,
            Self::High => 5,
            Self::VeryHigh => 6,
            Self::Maximum => 7,
        }
    }
}

impl PartialOrd for Magnitude {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Magnitude {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order_rank().cmp(&other.order_rank())
    }
}
