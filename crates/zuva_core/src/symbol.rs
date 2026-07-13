//! Compact identifiers for interned strings.
//!
//! Symbols allow the compiler to compare identifiers using small integer
//! values instead of repeatedly comparing and storing complete strings.
//!
//! A [`Symbol`] is meaningful only within the [`crate::StringInterner`] that
//! created it.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Compact identifier for a string stored in a string interner.
///
/// Symbols are assigned sequentially, beginning at zero. Two symbols are equal
/// only when their raw identifiers are equal.
///
/// A symbol from one interner must not be used to access another interner.
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct Symbol(u32);

impl Symbol {
    /// The first symbol identifier.
    pub const FIRST: Self = Self(0);

    /// Creates a symbol from its raw integer representation.
    ///
    /// This constructor does not verify that the symbol exists in a particular
    /// string interner.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw integer representation.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Converts the symbol into a collection index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// Returns the next sequential symbol.
    ///
    /// Returns `None` when the current symbol is `u32::MAX`.
    #[must_use]
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(raw) => Some(Self(raw)),
            None => None,
        }
    }
}

impl From<u32> for Symbol {
    fn from(value: u32) -> Self {
        Self::from_raw(value)
    }
}

impl From<Symbol> for u32 {
    fn from(value: Symbol) -> Self {
        value.as_u32()
    }
}

impl From<Symbol> for usize {
    fn from(value: Symbol) -> Self {
        value.index()
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "symbol#{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_symbol_from_raw_value() {
        let symbol = Symbol::from_raw(42);

        assert_eq!(symbol.as_u32(), 42);
        assert_eq!(symbol.index(), 42);
    }

    #[test]
    fn displays_symbol_identifier() {
        let symbol = Symbol::from_raw(7);

        assert_eq!(symbol.to_string(), "symbol#7");
    }

    #[test]
    fn returns_next_symbol() {
        let symbol = Symbol::from_raw(11);

        assert_eq!(symbol.checked_next(), Some(Symbol::from_raw(12)));
    }

    #[test]
    fn detects_symbol_overflow() {
        let symbol = Symbol::from_raw(u32::MAX);

        assert_eq!(symbol.checked_next(), None);
    }

    #[test]
    fn symbols_support_ordering() {
        let first = Symbol::from_raw(1);
        let second = Symbol::from_raw(2);

        assert!(first < second);
    }
}