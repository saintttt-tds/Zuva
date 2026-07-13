//! Source-file storage and source-map management.
//!
//! The compiler stores source positions as UTF-8 byte offsets. Line and column
//! numbers are calculated only when diagnostics are rendered.

use std::{
    fmt,
    path::{Path, PathBuf},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{
    error::{CoreError, CoreResult},
    span::{ByteOffset, Span},
};

/// Maximum source-file size supported by the current byte-offset representation.
///
/// Zuva currently uses 32-bit byte offsets. A single source file therefore
/// cannot exceed `u32::MAX` bytes.
pub const MAX_SOURCE_BYTES: usize = u32::MAX as usize;

/// A compact identifier assigned to a source file inside a [`SourceMap`].
///
/// Source identifiers are local to the source map that created them. A
/// `SourceId` from one source map must not be used with another source map.
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
pub struct SourceId(u32);

impl SourceId {
    /// Creates a source identifier from its raw integer representation.
    #[must_use]
    pub const fn from_raw(raw: u32) -> Self {
        Self(raw)
    }

    /// Returns the raw integer representation.
    #[must_use]
    pub const fn as_u32(self) -> u32 {
        self.0
    }

    /// Returns the source identifier as a collection index.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for SourceId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "source#{}", self.0)
    }
}

/// A source file registered with the compiler.
///
/// Source text is immutable after registration. This allows spans and
/// diagnostics to remain valid throughout a compilation session.
#[derive(Clone, Debug)]
pub struct SourceFile {
    id: SourceId,
    name: SmolStr,
    path: Option<PathBuf>,
    text: Arc<str>,
    line_starts: Vec<ByteOffset>,
}

impl SourceFile {
    /// Creates a source file and builds its line-start index.
    pub(crate) fn new(
        id: SourceId,
        name: impl Into<SmolStr>,
        path: Option<PathBuf>,
        text: impl Into<Arc<str>>,
    ) -> CoreResult<Self> {
        let name = name.into();
        let text = text.into();
        let byte_length = text.len();

        if byte_length > MAX_SOURCE_BYTES {
            return Err(CoreError::SourceTooLarge {
                name: name.to_string(),
                bytes: byte_length,
                maximum: MAX_SOURCE_BYTES,
            });
        }

        let mut line_starts = Vec::with_capacity(estimate_line_count(&text));
        line_starts.push(ByteOffset::ZERO);

        for (index, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                let next_byte = index + 1;
                let raw_offset =
                    u32::try_from(next_byte).map_err(|_| CoreError::SourceTooLarge {
                        name: name.to_string(),
                        bytes: byte_length,
                        maximum: MAX_SOURCE_BYTES,
                    })?;

                line_starts.push(ByteOffset::new(raw_offset));
            }
        }

        Ok(Self {
            id,
            name,
            path,
            text,
            line_starts,
        })
    }

    /// Returns the source file's identifier.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the source file's display name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the source file's filesystem path, when available.
    #[must_use]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Returns the complete UTF-8 source text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the source length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Returns `true` when the source file contains no text.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Returns the number of logical lines in the source file.
    ///
    /// An empty source file contains one logical line. A file ending in a
    /// newline contains a final empty logical line.
    #[must_use]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    /// Returns the byte offset at which a zero-based line begins.
    #[must_use]
    pub fn line_start(&self, line_index: usize) -> Option<ByteOffset> {
        self.line_starts.get(line_index).copied()
    }

    /// Returns the zero-based line index containing the supplied byte offset.
    ///
    /// An offset equal to the source length is accepted because it represents
    /// the end-of-file position.
    #[must_use]
    pub fn line_index(&self, offset: ByteOffset) -> Option<usize> {
        if offset.to_usize() > self.len() {
            return None;
        }

        match self.line_starts.binary_search(&offset) {
            Ok(index) => Some(index),
            Err(0) => Some(0),
            Err(index) => Some(index - 1),
        }
    }

    /// Converts a byte offset into a zero-based `(line, column)` location.
    ///
    /// The returned column is measured in Unicode scalar values rather than
    /// bytes. The method returns `None` when the offset is outside the source
    /// or is not located on a valid UTF-8 character boundary.
    #[must_use]
    pub fn line_column(&self, offset: ByteOffset) -> Option<(usize, usize)> {
        let line_index = self.line_index(offset)?;
        let line_start = self.line_start(line_index)?.to_usize();
        let byte_index = offset.to_usize();

        let prefix = self.text.get(line_start..byte_index)?;
        let column = prefix.chars().count();

        Some((line_index, column))
    }

    /// Converts a byte offset into a one-based display location.
    ///
    /// This is intended for user-facing diagnostics.
    #[must_use]
    pub fn display_line_column(&self, offset: ByteOffset) -> Option<(usize, usize)> {
        self.line_column(offset)
            .map(|(line, column)| (line + 1, column + 1))
    }

    /// Returns a line without its terminating newline characters.
    ///
    /// The line index is zero-based.
    #[must_use]
    pub fn line_text(&self, line_index: usize) -> Option<&str> {
        let start = self.line_start(line_index)?.to_usize();

        let end = self
            .line_start(line_index + 1)
            .map_or_else(|| self.len(), ByteOffset::to_usize);

        let line = self.text.get(start..end)?;
        Some(line.trim_end_matches(['\r', '\n']))
    }

    /// Returns the source text covered by a span.
    ///
    /// The span must belong to this source file and both offsets must be valid
    /// UTF-8 boundaries.
    #[must_use]
    pub fn slice(&self, span: Span) -> Option<&str> {
        if span.source() != self.id {
            return None;
        }

        let start = span.start().to_usize();
        let end = span.end().to_usize();

        if start > end || end > self.len() {
            return None;
        }

        self.text.get(start..end)
    }

    /// Returns `true` when a span belongs to this file and is within bounds.
    #[must_use]
    pub fn contains_span(&self, span: Span) -> bool {
        span.source() == self.id
            && span.start() <= span.end()
            && span.end().to_usize() <= self.len()
            && self.text.is_char_boundary(span.start().to_usize())
            && self.text.is_char_boundary(span.end().to_usize())
    }
}

/// Collection of all source files participating in a compilation session.
#[derive(Clone, Debug, Default)]
pub struct SourceMap {
    sources: Vec<SourceFile>,
}

impl SourceMap {
    /// Creates an empty source map.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            sources: Vec::new(),
        }
    }

    /// Returns the number of registered source files.
    #[must_use]
    pub fn len(&self) -> usize {
        self.sources.len()
    }

    /// Returns `true` when no source files have been registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.sources.is_empty()
    }

    /// Registers an in-memory source file.
    ///
    /// Virtual sources are used by the REPL, tests, generated source code, and
    /// editor integrations.
    pub fn add_virtual(
        &mut self,
        name: impl Into<SmolStr>,
        text: impl Into<Arc<str>>,
    ) -> CoreResult<SourceId> {
        self.insert(name, None, text)
    }

    /// Registers source text associated with a filesystem path.
    ///
    /// This method does not read the file from disk. The caller supplies the
    /// source text so filesystem access remains outside the core source map.
    pub fn add_file(
        &mut self,
        path: impl Into<PathBuf>,
        text: impl Into<Arc<str>>,
    ) -> CoreResult<SourceId> {
        let path = path.into();

        let name = path.file_name().map_or_else(
            || path.to_string_lossy().into_owned(),
            |file_name| file_name.to_string_lossy().into_owned(),
        );

        self.insert(name, Some(path), text)
    }

    /// Returns a source file by identifier.
    #[must_use]
    pub fn get(&self, source_id: SourceId) -> Option<&SourceFile> {
        self.sources.get(source_id.index())
    }

    /// Returns an iterator over all registered source files.
    pub fn iter(&self) -> impl ExactSizeIterator<Item = &SourceFile> {
        self.sources.iter()
    }

    /// Returns the complete text of a registered source file.
    #[must_use]
    pub fn source_text(&self, source_id: SourceId) -> Option<&str> {
        self.get(source_id).map(SourceFile::text)
    }

    /// Returns the text covered by a source span.
    #[must_use]
    pub fn slice(&self, span: Span) -> Option<&str> {
        self.get(span.source())?.slice(span)
    }

    /// Converts a source position into a one-based user-facing location.
    #[must_use]
    pub fn display_line_column(
        &self,
        source_id: SourceId,
        offset: ByteOffset,
    ) -> Option<(usize, usize)> {
        self.get(source_id)?.display_line_column(offset)
    }

    fn insert(
        &mut self,
        name: impl Into<SmolStr>,
        path: Option<PathBuf>,
        text: impl Into<Arc<str>>,
    ) -> CoreResult<SourceId> {
        let raw_id =
            u32::try_from(self.sources.len()).map_err(|_| CoreError::TooManySources {
                maximum: u32::MAX,
            })?;

        let source_id = SourceId::from_raw(raw_id);
        let source = SourceFile::new(source_id, name, path, text)?;

        self.sources.push(source);

        Ok(source_id)
    }
}

impl<'a> IntoIterator for &'a SourceMap {
    type Item = &'a SourceFile;
    type IntoIter = std::slice::Iter<'a, SourceFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.sources.iter()
    }
}

fn estimate_line_count(text: &str) -> usize {
    text.bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        .saturating_add(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_virtual_source() {
        let mut source_map = SourceMap::new();

        let source_id = source_map
            .add_virtual("example.zuva", "let value = 42\n")
            .expect("the source should be registered");

        let source = source_map
            .get(source_id)
            .expect("the source should exist");

        assert_eq!(source.name(), "example.zuva");
        assert_eq!(source.text(), "let value = 42\n");
        assert_eq!(source.line_count(), 2);
    }

    #[test]
    fn calculates_unicode_line_and_column() {
        let mut source_map = SourceMap::new();

        let source_id = source_map
            .add_virtual("unicode.zuva", "let greeting = \"Zuva\"\nprint(\"Mhoro\")")
            .expect("the source should be registered");

        let source = source_map
            .get(source_id)
            .expect("the source should exist");

        let offset = ByteOffset::new(27);

        assert_eq!(source.display_line_column(offset), Some((2, 6)));
    }

    #[test]
    fn returns_line_text_without_line_terminator() {
        let mut source_map = SourceMap::new();

        let source_id = source_map
            .add_virtual("lines.zuva", "first\r\nsecond\nthird")
            .expect("the source should be registered");

        let source = source_map
            .get(source_id)
            .expect("the source should exist");

        assert_eq!(source.line_text(0), Some("first"));
        assert_eq!(source.line_text(1), Some("second"));
        assert_eq!(source.line_text(2), Some("third"));
    }

    #[test]
    fn rejects_spans_from_another_source() {
        let mut source_map = SourceMap::new();

        let first = source_map
            .add_virtual("first.zuva", "first")
            .expect("the first source should be registered");

        let second = source_map
            .add_virtual("second.zuva", "second")
            .expect("the second source should be registered");

        let span = Span::new(first, ByteOffset::new(0), ByteOffset::new(5));

        let second_source = source_map
            .get(second)
            .expect("the second source should exist");

        assert_eq!(second_source.slice(span), None);
    }
}