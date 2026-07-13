//! String interning for identifiers and frequently repeated compiler text.
//!
//! A string interner stores each unique string once and assigns it a compact
//! [`Symbol`]. Compiler stages can then compare symbols instead of repeatedly
//! comparing complete strings.
//!
//! Symbols are local to the interner that created them. A symbol from one
//! interner must not be resolved using another interner.

use std::collections::HashMap;

use smol_str::SmolStr;

use crate::symbol::Symbol;

/// Stores unique strings and assigns each one a compact [`Symbol`].
///
/// The interner maintains two synchronized collections:
///
/// - a hash map for string-to-symbol lookup;
/// - a vector for symbol-to-string resolution.
///
/// Strings are assigned symbols sequentially, beginning with
/// [`Symbol::FIRST`].
#[derive(Clone, Debug, Default)]
pub struct StringInterner {
    symbols: HashMap<SmolStr, Symbol>,
    strings: Vec<SmolStr>,
}

impl StringInterner {
    /// Creates an empty string interner.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates an empty interner with capacity for at least `capacity`
    /// unique strings.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            symbols: HashMap::with_capacity(capacity),
            strings: Vec::with_capacity(capacity),
        }
    }

    /// Returns the number of unique interned strings.
    #[must_use]
    pub fn len(&self) -> usize {
        self.strings.len()
    }

    /// Returns `true` when no strings have been interned.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.strings.is_empty()
    }

    /// Returns the current storage capacity of the symbol table.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.strings.capacity()
    }

    /// Removes all interned strings.
    ///
    /// Existing symbols become invalid after this method is called.
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.strings.clear();
    }

    /// Returns the symbol already assigned to a string.
    ///
    /// This method does not add the string when it is absent.
    #[must_use]
    pub fn get(&self, value: &str) -> Option<Symbol> {
        self.symbols.get(value).copied()
    }

    /// Returns `true` when the supplied string has already been interned.
    #[must_use]
    pub fn contains(&self, value: &str) -> bool {
        self.symbols.contains_key(value)
    }

    /// Interns a string and returns its symbol.
    ///
    /// When the string already exists, its existing symbol is returned.
    ///
    /// Returns `None` only when the interner has exhausted all possible
    /// `u32` symbol identifiers.
    pub fn intern(&mut self, value: impl AsRef<str>) -> Option<Symbol> {
        let value = value.as_ref();

        if let Some(symbol) = self.get(value) {
            return Some(symbol);
        }

        let raw_symbol = u32::try_from(self.strings.len()).ok()?;
        let symbol = Symbol::from_raw(raw_symbol);
        let owned_value = SmolStr::new(value);

        self.strings.push(owned_value.clone());
        self.symbols.insert(owned_value, symbol);

        Some(symbol)
    }

    /// Interns several strings in their iteration order.
    ///
    /// Existing strings retain their original symbols. Returns `None` if the
    /// interner exhausts the available `u32` symbol space.
    pub fn intern_all<I, S>(&mut self, values: I) -> Option<Vec<Symbol>>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        values
            .into_iter()
            .map(|value| self.intern(value))
            .collect()
    }

    /// Resolves a symbol to its interned string.
    #[must_use]
    pub fn resolve(&self, symbol: Symbol) -> Option<&str> {
        self.strings.get(symbol.index()).map(SmolStr::as_str)
    }

    /// Resolves a symbol to its stored [`SmolStr`].
    #[must_use]
    pub fn resolve_smol(&self, symbol: Symbol) -> Option<&SmolStr> {
        self.strings.get(symbol.index())
    }

    /// Returns an iterator over all interned strings in symbol order.
    pub fn strings(&self) -> impl ExactSizeIterator<Item = &str> {
        self.strings.iter().map(SmolStr::as_str)
    }

    /// Returns an iterator over every symbol and its associated string.
    pub fn iter(&self) -> impl Iterator<Item = (Symbol, &str)> {
        (0_u32..)
            .zip(self.strings.iter())
            .map(|(raw_symbol, value)| {
                (Symbol::from_raw(raw_symbol), value.as_str())
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_empty_interner() {
        let interner = StringInterner::new();

        assert!(interner.is_empty());
        assert_eq!(interner.len(), 0);
    }

    #[test]
    fn interns_new_string() {
        let mut interner = StringInterner::new();

        let symbol = interner
            .intern("value")
            .expect("the symbol space should not be exhausted");

        assert_eq!(symbol, Symbol::FIRST);
        assert_eq!(interner.len(), 1);
        assert_eq!(interner.resolve(symbol), Some("value"));
    }

    #[test]
    fn returns_existing_symbol_for_duplicate_string() {
        let mut interner = StringInterner::new();

        let first = interner
            .intern("customer")
            .expect("the symbol should be created");

        let second = interner
            .intern("customer")
            .expect("the existing symbol should be returned");

        assert_eq!(first, second);
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn assigns_symbols_sequentially() {
        let mut interner = StringInterner::new();

        let first = interner
            .intern("first")
            .expect("the first symbol should be created");

        let second = interner
            .intern("second")
            .expect("the second symbol should be created");

        let third = interner
            .intern("third")
            .expect("the third symbol should be created");

        assert_eq!(first.as_u32(), 0);
        assert_eq!(second.as_u32(), 1);
        assert_eq!(third.as_u32(), 2);
    }

    #[test]
    fn looks_up_existing_string_without_interning() {
        let mut interner = StringInterner::new();

        let symbol = interner
            .intern("identifier")
            .expect("the symbol should be created");

        assert_eq!(interner.get("identifier"), Some(symbol));
        assert_eq!(interner.get("missing"), None);
        assert_eq!(interner.len(), 1);
    }

    #[test]
    fn resolves_invalid_symbol_as_none() {
        let interner = StringInterner::new();
        let invalid = Symbol::from_raw(100);

        assert_eq!(interner.resolve(invalid), None);
    }

    #[test]
    fn interns_multiple_strings() {
        let mut interner = StringInterner::new();

        let symbols = interner
            .intern_all(["let", "value", "let"])
            .expect("the symbols should be created");

        assert_eq!(symbols.len(), 3);
        assert_eq!(symbols[0], symbols[2]);
        assert_ne!(symbols[0], symbols[1]);
        assert_eq!(interner.len(), 2);
    }

    #[test]
    fn iterates_in_symbol_order() {
        let mut interner = StringInterner::new();

        interner
            .intern_all(["alpha", "beta", "gamma"])
            .expect("the symbols should be created");

        let entries = interner.iter().collect::<Vec<_>>();

        assert_eq!(
            entries,
            vec![
                (Symbol::from_raw(0), "alpha"),
                (Symbol::from_raw(1), "beta"),
                (Symbol::from_raw(2), "gamma"),
            ]
        );
    }

    #[test]
    fn clearing_invalidates_existing_symbols() {
        let mut interner = StringInterner::new();

        let symbol = interner
            .intern("temporary")
            .expect("the symbol should be created");

        interner.clear();

        assert!(interner.is_empty());
        assert_eq!(interner.resolve(symbol), None);
    }
}