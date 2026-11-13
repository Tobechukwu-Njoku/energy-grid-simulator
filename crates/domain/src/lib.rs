//! Shared identifiers, units and domain contracts (ADR 0001, ADR 0002).
//!
//! This crate has no database, network or solver dependencies. M0 defines only
//! the identity and classification types other crates need to compile their
//! contracts; network records and revisions follow in M1.

#![forbid(unsafe_code)]

use std::fmt;
use std::str::FromStr;

/// Opaque, permanent identity of a domain entity (ADR 0002).
///
/// The UUID never encodes a mutable name, owner, geography or row position.
/// A rename keeps the identity; a split, merger or replacement creates new ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct EntityId(uuid::Uuid);

impl EntityId {
    /// Wraps an existing UUID. The nil UUID is rejected because it cannot
    /// identify an entity.
    pub fn from_uuid(value: uuid::Uuid) -> Result<Self, ParseIdError> {
        if value.is_nil() {
            Err(ParseIdError::Nil)
        } else {
            Ok(Self(value))
        }
    }

    /// The underlying UUID.
    #[must_use]
    pub fn as_uuid(&self) -> &uuid::Uuid {
        &self.0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.hyphenated().fmt(formatter)
    }
}

impl FromStr for EntityId {
    type Err = ParseIdError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let value = uuid::Uuid::try_parse(text).map_err(|_| ParseIdError::Malformed)?;
        Self::from_uuid(value)
    }
}

/// Why an [`EntityId`] could not be created.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseIdError {
    /// The text is not a UUID.
    Malformed,
    /// The nil UUID is not a valid entity identity.
    Nil,
}

impl fmt::Display for ParseIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Malformed => "entity id is not a UUID",
            Self::Nil => "entity id must not be the nil UUID",
        })
    }
}

impl std::error::Error for ParseIdError {}

/// Native settlement currency of a market (D-38).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Currency {
    /// Pound sterling (GB).
    Gbp,
    /// Euro (I-SEM).
    Eur,
}

impl Currency {
    /// ISO 4217 code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Gbp => "GBP",
            Self::Eur => "EUR",
        }
    }
}

impl fmt::Display for Currency {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

/// P1 market areas (D-18). A market area is not an electrical island (ADR 0002).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarketArea {
    /// Great Britain.
    GreatBritain,
    /// Single Electricity Market of Ireland and Northern Ireland.
    Isem,
}

impl MarketArea {
    /// The market's native currency (D-38).
    #[must_use]
    pub const fn currency(self) -> Currency {
        match self {
            Self::GreatBritain => Currency::Gbp,
            Self::Isem => Currency::Eur,
        }
    }
}

/// Dataset licence tag (D-56, FR-19). Unknown commercial permission is
/// `ResearchOnly`, never presumed cleared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LicenceTag {
    /// Licence permits commercial use.
    OpenCommercial,
    /// Usable for non-commercial research only; raw data is never committed.
    ResearchOnly,
}

impl LicenceTag {
    /// Parses the exact manifest spelling. Any other value is rejected rather
    /// than defaulted.
    pub fn parse_manifest(text: &str) -> Option<Self> {
        match text {
            "open-commercial" => Some(Self::OpenCommercial),
            "research-only" => Some(Self::ResearchOnly),
            _ => None,
        }
    }

    /// The tag of a derivative: research-only if any ancestor is.
    #[must_use]
    pub fn combine(self, other: Self) -> Self {
        if self == Self::OpenCommercial && other == Self::OpenCommercial {
            Self::OpenCommercial
        } else {
            Self::ResearchOnly
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_id_round_trips_and_rejects_nil() {
        let text = "6f1c2a4e-8d0b-4b8e-9a3f-2c5d7e9f1a0b";
        let id: EntityId = text.parse().expect("valid uuid");
        assert_eq!(id.to_string(), text);
        assert_eq!(
            "00000000-0000-0000-0000-000000000000".parse::<EntityId>(),
            Err(ParseIdError::Nil)
        );
        assert_eq!("bus-17".parse::<EntityId>(), Err(ParseIdError::Malformed));
    }

    #[test]
    fn market_currency_is_native() {
        assert_eq!(MarketArea::GreatBritain.currency(), Currency::Gbp);
        assert_eq!(MarketArea::Isem.currency().code(), "EUR");
    }

    #[test]
    fn licence_tags_are_exact_and_taint_derivatives() {
        assert_eq!(
            LicenceTag::parse_manifest("open-commercial"),
            Some(LicenceTag::OpenCommercial)
        );
        assert_eq!(LicenceTag::parse_manifest("Open-Commercial"), None);
        assert_eq!(LicenceTag::parse_manifest("unknown"), None);
        assert_eq!(
            LicenceTag::OpenCommercial.combine(LicenceTag::ResearchOnly),
            LicenceTag::ResearchOnly
        );
        assert_eq!(
            LicenceTag::OpenCommercial.combine(LicenceTag::OpenCommercial),
            LicenceTag::OpenCommercial
        );
    }
}
