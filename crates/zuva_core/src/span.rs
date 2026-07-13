//! Byte-based source positions and spans.
//!
//! Zuva stores source locations as UTF-8 byte offsets rather than line and
//! column numbers. Line and column information is calculated only when a
//! diagnostic is displayed.
//!
//! Spans use half-open ranges:
//!
//! ```text
//! [start, end)
//! ```
//!
//! The start offset is included and the end offset is excluded.

use std::{
    fmt,
    ops::Range,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::source::SourceId;

/// A zero-based byte offset inside a UTF-8 source file.
///
/// A `ByteOffset` identifies a position between bytes. It does not guarantee
/// that the position is a valid UTF-8 character boundary; that validation is
/// performed by [`crate::source::SourceFile`].
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[serde(transparent)]
pub struct ByteOffset(u32);

impl ByteOffset {
    /// The first byte position in a source file.
    pub const ZERO: Self = Self(0);

    /// Creates a byte offset from its raw `u32` representation.
    #[must_use]
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw `u32` representation.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Converts the byte offset into a collection index.
    #[must_use]
    pub const fn to_usize(self) -> usize {
        self.0 as usize
    }

    /// Returns a new offset advanced by the supplied number of bytes.
    ///
    /// Returns `None` if the addition would overflow the `u32`
    /// representation.
    #[must_use]
    pub const fn checked_add(self, bytes: u32) -> Option<Self> {
        match self.0.checked_add(bytes) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns a new offset moved backward by the supplied number of bytes.
    ///
    /// Returns `None` if the subtraction would move before byte zero.
    #[must_use]
    pub const fn checked_sub(self, bytes: u32) -> Option<Self> {
        match self.0.checked_sub(bytes) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Returns the distance in bytes from `self` to `other`.
    ///
    /// Returns `None` when `other` is positioned before `self`.
    #[must_use]
    pub const fn distance_to(self, other: Self) -> Option<u32> {
        other.0.checked_sub(self.0)
    }
}

impl From<u32> for ByteOffset {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<ByteOffset> for u32 {
    fn from(value: ByteOffset) -> Self {
        value.as_u32()
    }
}

impl From<ByteOffset> for usize {
    fn from(value: ByteOffset) -> Self {
        value.to_usize()
    }
}

impl fmt::Display for ByteOffset {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A half-open byte range associated with one source file.
///
/// A span includes its starting byte and excludes its ending byte.
///
/// For example, the span `[4, 7)` covers bytes `4`, `5`, and `6`.
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
pub struct Span {
    source: SourceId,
    start: ByteOffset,
    end: ByteOffset,
}

impl Span {
    /// Creates a source span.
    ///
    /// # Panics
    ///
    /// Panics when `start` is greater than `end`.
    ///
    /// Compiler components processing untrusted offsets should use
    /// [`Span::checked`] instead.
    #[must_use]
    pub const fn new(
        source: SourceId,
        start: ByteOffset,
        end: ByteOffset,
    ) -> Self {
        assert!(
            start.as_u32() <= end.as_u32(),
            "a span cannot end before it starts"
        );

        Self {
            source,
            start,
            end,
        }
    }

    /// Creates a span when the supplied byte range is valid.
    ///
    /// Returns `None` when `start` is greater than `end`.
    #[must_use]
    pub const fn checked(
        source: SourceId,
        start: ByteOffset,
        end: ByteOffset,
    ) -> Option<Self> {
        if start.as_u32() <= end.as_u32() {
            Some(Self {
                source,
                start,
                end,
            })
        } else {
            None
        }
    }

    /// Creates an empty span at a single byte position.
    #[must_use]
    pub const fn empty(source: SourceId, position: ByteOffset) -> Self {
        Self {
            source,
            start: position,
            end: position,
        }
    }

    /// Creates a span from a standard byte range.
    ///
    /// Returns `None` if either range boundary does not fit into `u32`, or if
    /// the range is reversed.
    #[must_use]
    pub fn from_range(
        source: SourceId,
        range: Range<usize>,
    ) -> Option<Self> {
        let start = u32::try_from(range.start).ok()?;
        let end = u32::try_from(range.end).ok()?;

        Self::checked(
            source,
            ByteOffset::new(start),
            ByteOffset::new(end),
        )
    }

    /// Returns the source file associated with this span.
    #[must_use]
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the inclusive starting byte offset.
    #[must_use]
    pub const fn start(self) -> ByteOffset {
        self.start
    }

    /// Returns the exclusive ending byte offset.
    #[must_use]
    pub const fn end(self) -> ByteOffset {
        self.end
    }

    /// Returns the length of the span in bytes.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.end.as_u32() - self.start.as_u32()
    }

    /// Returns `true` when the span covers no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start.as_u32() == self.end.as_u32()
    }

    /// Returns this span as a standard byte range.
    #[must_use]
    pub const fn byte_range(self) -> Range<usize> {
        self.start.to_usize()..self.end.to_usize()
    }

    /// Returns `true` when the byte position is inside this span.
    ///
    /// Because spans are half-open, the ending offset is not included.
    #[must_use]
    pub const fn contains(self, position: ByteOffset) -> bool {
        self.start.as_u32() <= position.as_u32()
            && position.as_u32() < self.end.as_u32()
    }

    /// Returns `true` when this span completely contains another span.
    ///
    /// Spans belonging to different source files never contain one another.
    #[must_use]
    pub const fn contains_span(self, other: Self) -> bool {
        self.source.as_u32() == other.source.as_u32()
            && self.start.as_u32() <= other.start.as_u32()
            && self.end.as_u32() >= other.end.as_u32()
    }

    /// Returns `true` when this span overlaps another span.
    ///
    /// Adjacent spans do not overlap. Spans from different source files never
    /// overlap.
    #[must_use]
    pub const fn overlaps(self, other: Self) -> bool {
        self.source.as_u32() == other.source.as_u32()
            && self.start.as_u32() < other.end.as_u32()
            && other.start.as_u32() < self.end.as_u32()
    }

    /// Returns `true` when two spans touch without overlapping.
    ///
    /// Spans from different source files are never considered adjacent.
    #[must_use]
    pub const fn is_adjacent_to(self, other: Self) -> bool {
        self.source.as_u32() == other.source.as_u32()
            && (self.end.as_u32() == other.start.as_u32()
                || other.end.as_u32() == self.start.as_u32())
    }

    /// Returns the smallest span covering both spans.
    ///
    /// Returns `None` when the spans belong to different source files.
    #[must_use]
    pub const fn join(self, other: Self) -> Option<Self> {
        if self.source.as_u32() != other.source.as_u32() {
            return None;
        }

        let start = if self.start.as_u32() <= other.start.as_u32() {
            self.start
        } else {
            other.start
        };

        let end = if self.end.as_u32() >= other.end.as_u32() {
            self.end
        } else {
            other.end
        };

        Some(Self {
            source: self.source,
            start,
            end,
        })
    }

    /// Returns the intersection of two spans.
    ///
    /// Returns `None` when the spans belong to different source files or do
    /// not overlap.
    #[must_use]
    pub const fn intersection(self, other: Self) -> Option<Self> {
        if self.source.as_u32() != other.source.as_u32() {
            return None;
        }

        let start = if self.start.as_u32() >= other.start.as_u32() {
            self.start
        } else {
            other.start
        };

        let end = if self.end.as_u32() <= other.end.as_u32() {
            self.end
        } else {
            other.end
        };

        if start.as_u32() < end.as_u32() {
            Some(Self {
                source: self.source,
                start,
                end,
            })
        } else {
            None
        }
    }

    /// Returns a span with the same source and start position but a new end.
    ///
    /// Returns `None` when the new end is positioned before the current start.
    #[must_use]
    pub const fn with_end(self, end: ByteOffset) -> Option<Self> {
        Self::checked(self.source, self.start, end)
    }

    /// Returns a span with the same source and end position but a new start.
    ///
    /// Returns `None` when the new start is positioned after the current end.
    #[must_use]
    pub const fn with_start(self, start: ByteOffset) -> Option<Self> {
        Self::checked(self.source, start, self.end)
    }

    /// Returns an empty span positioned at this span's start.
    #[must_use]
    pub const fn start_point(self) -> Self {
        Self::empty(self.source, self.start)
    }

    /// Returns an empty span positioned at this span's end.
    #[must_use]
    pub const fn end_point(self) -> Self {
        Self::empty(self.source, self.end)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}:{}..{}",
            self.source,
            self.start,
            self.end
        )
    }
}

/// A value associated with its original source span.
///
/// `Spanned<T>` is used for tokens, syntax nodes, identifiers, literals, and
/// other compiler values that must retain source-location information.
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
pub struct Spanned<T> {
    value: T,
    span: Span,
}

impl<T> Spanned<T> {
    /// Associates a value with a source span.
    #[must_use]
    pub const fn new(value: T, span: Span) -> Self {
        Self {
            value,
            span,
        }
    }

    /// Returns a shared reference to the wrapped value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the wrapped value.
    #[must_use]
    pub const fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Returns the source span.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Replaces the source span while preserving the wrapped value.
    #[must_use]
    pub fn with_span(self, span: Span) -> Self {
        Self {
            value: self.value,
            span,
        }
    }

    /// Converts the wrapped value while preserving the source span.
    #[must_use]
    pub fn map<U>(
        self,
        mapper: impl FnOnce(T) -> U,
    ) -> Spanned<U> {
        Spanned {
            value: mapper(self.value),
            span: self.span,
        }
    }

    /// Returns a spanned shared reference to the wrapped value.
    #[must_use]
    pub const fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            value: &self.value,
            span: self.span,
        }
    }

    /// Returns a spanned mutable reference to the wrapped value.
    #[must_use]
    pub fn as_mut(&mut self) -> Spanned<&mut T> {
        Spanned {
            value: &mut self.value,
            span: self.span,
        }
    }

    /// Separates the wrapped value and its source span.
    #[must_use]
    pub fn into_parts(self) -> (T, Span) {
        (self.value, self.span)
    }

    /// Consumes the wrapper and returns the contained value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_id(raw: u32) -> SourceId {
        SourceId::from_raw(raw)
    }

    #[test]
    fn calculates_offset_distance() {
        let start = ByteOffset::new(4);
        let end = ByteOffset::new(12);

        assert_eq!(start.distance_to(end), Some(8));
        assert_eq!(end.distance_to(start), None);
    }

    #[test]
    fn creates_valid_span() {
        let span = Span::new(
            source_id(0),
            ByteOffset::new(5),
            ByteOffset::new(12),
        );

        assert_eq!(span.start(), ByteOffset::new(5));
        assert_eq!(span.end(), ByteOffset::new(12));
        assert_eq!(span.len(), 7);
        assert!(!span.is_empty());
    }

    #[test]
    fn rejects_reversed_checked_span() {
        let span = Span::checked(
            source_id(0),
            ByteOffset::new(12),
            ByteOffset::new(5),
        );

        assert_eq!(span, None);
    }

    #[test]
    fn uses_half_open_range_semantics() {
        let span = Span::new(
            source_id(0),
            ByteOffset::new(3),
            ByteOffset::new(6),
        );

        assert!(span.contains(ByteOffset::new(3)));
        assert!(span.contains(ByteOffset::new(5)));
        assert!(!span.contains(ByteOffset::new(6)));
    }

    #[test]
    fn joins_spans_from_same_source() {
        let first = Span::new(
            source_id(0),
            ByteOffset::new(2),
            ByteOffset::new(5),
        );

        let second = Span::new(
            source_id(0),
            ByteOffset::new(8),
            ByteOffset::new(12),
        );

        let joined = first
            .join(second)
            .expect("spans from the same source should join");

        assert_eq!(joined.start(), ByteOffset::new(2));
        assert_eq!(joined.end(), ByteOffset::new(12));
    }

    #[test]
    fn does_not_join_different_sources() {
        let first = Span::new(
            source_id(0),
            ByteOffset::new(0),
            ByteOffset::new(5),
        );

        let second = Span::new(
            source_id(1),
            ByteOffset::new(0),
            ByteOffset::new(5),
        );

        assert_eq!(first.join(second), None);
    }

    #[test]
    fn calculates_span_intersection() {
        let first = Span::new(
            source_id(0),
            ByteOffset::new(2),
            ByteOffset::new(8),
        );

        let second = Span::new(
            source_id(0),
            ByteOffset::new(5),
            ByteOffset::new(12),
        );

        let intersection = first
            .intersection(second)
            .expect("the spans should overlap");

        assert_eq!(intersection.start(), ByteOffset::new(5));
        assert_eq!(intersection.end(), ByteOffset::new(8));
    }

    #[test]
    fn maps_spanned_value_without_losing_span() {
        let span = Span::new(
            source_id(0),
            ByteOffset::new(1),
            ByteOffset::new(4),
        );

        let value = Spanned::new("42", span);
        let parsed = value.map(str::parse::<u32>);

        assert_eq!(parsed.span(), span);
        assert_eq!(parsed.into_inner(), Ok(42));
    }
}