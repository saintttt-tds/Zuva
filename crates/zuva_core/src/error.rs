//! Errors produced by shared Zuva compiler infrastructure.
//!
//! These errors describe failures in source management, source-position
//! conversion, and other low-level compiler services. User-facing syntax,
//! semantic, and runtime errors are represented by higher-level diagnostics.

use thiserror::Error;

use crate::{
    source::SourceId,
    span::Span,
};

/// Result type used by `zuva_core`.
pub type CoreResult<T> = Result<T, CoreError>;

/// Errors produced by the Zuva core infrastructure.
#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CoreError {
    /// A source file exceeds the maximum size supported by the compiler.
    #[error(
        "source file `{name}` contains {bytes} bytes, exceeding the maximum of {maximum} bytes"
    )]
    SourceTooLarge {
        /// Display name of the source file.
        name: String,

        /// Actual source-file size in bytes.
        bytes: usize,

        /// Maximum supported source-file size in bytes.
        maximum: usize,
    },

    /// The source map cannot allocate another source identifier.
    #[error(
        "the source map cannot contain more than {maximum} source files"
    )]
    TooManySources {
        /// Maximum number of source files supported by the identifier type.
        maximum: u32,
    },

    /// A source identifier does not exist in the current source map.
    #[error(
        "source identifier {source_id} is invalid; the source map contains {available} source files"
    )]
    InvalidSourceId {
        /// Invalid source identifier.
        source_id: SourceId,

        /// Number of source files currently registered.
        available: usize,
    },

    /// A source span is outside the boundaries of its source file.
    #[error(
        "source span {span} is outside the source file boundary of {source_length} bytes"
    )]
    InvalidSpan {
        /// Invalid source span.
        span: Span,

        /// Length of the associated source file in bytes.
        source_length: usize,
    },

    /// A byte offset cannot be represented by the compiler's offset type.
    #[error(
        "byte offset {value} exceeds the maximum supported value of {}",
        u32::MAX
    )]
    ByteOffsetOverflow {
        /// Byte offset that could not be represented.
        value: usize,
    },

    /// A source name is empty.
    #[error("source files must have a non-empty display name")]
    EmptySourceName,

    /// A requested byte position is not a valid UTF-8 boundary.
    #[error(
        "byte offset {offset} in source {source_id} is not a valid UTF-8 character boundary"
    )]
    InvalidUtf8Boundary {
        /// Source file containing the invalid position.
        source_id: SourceId,

        /// Invalid byte offset.
        offset: u32,
    },
}

impl CoreError {
    /// Returns a stable internal error code.
    ///
    /// Core error codes use the `ZC` prefix:
    ///
    /// - `ZC0001`: source file too large;
    /// - `ZC0002`: too many source files;
    /// - `ZC0003`: invalid source identifier;
    /// - `ZC0004`: invalid source span;
    /// - `ZC0005`: byte-offset overflow;
    /// - `ZC0006`: empty source name;
    /// - `ZC0007`: invalid UTF-8 boundary.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::SourceTooLarge { .. } => "ZC0001",
            Self::TooManySources { .. } => "ZC0002",
            Self::InvalidSourceId { .. } => "ZC0003",
            Self::InvalidSpan { .. } => "ZC0004",
            Self::ByteOffsetOverflow { .. } => "ZC0005",
            Self::EmptySourceName => "ZC0006",
            Self::InvalidUtf8Boundary { .. } => "ZC0007",
        }
    }

    /// Returns `true` when the error resulted from invalid source input.
    #[must_use]
    pub const fn is_source_error(&self) -> bool {
        matches!(
            self,
            Self::SourceTooLarge { .. }
                | Self::InvalidSourceId { .. }
                | Self::InvalidSpan { .. }
                | Self::EmptySourceName
                | Self::InvalidUtf8Boundary { .. }
        )
    }

    /// Returns `true` when the error represents an internal capacity limit.
    #[must_use]
    pub const fn is_capacity_error(&self) -> bool {
        matches!(
            self,
            Self::SourceTooLarge { .. }
                | Self::TooManySources { .. }
                | Self::ByteOffsetOverflow { .. }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_stable_error_code() {
        let error = CoreError::SourceTooLarge {
            name: "large.zuva".to_owned(),
            bytes: 5_000,
            maximum: 1_000,
        };

        assert_eq!(error.code(), "ZC0001");
    }

    #[test]
    fn formats_source_size_error() {
        let error = CoreError::SourceTooLarge {
            name: "large.zuva".to_owned(),
            bytes: 5_000,
            maximum: 1_000,
        };

        assert_eq!(
            error.to_string(),
            "source file `large.zuva` contains 5000 bytes, exceeding the maximum of 1000 bytes"
        );
    }

    #[test]
    fn classifies_capacity_errors() {
        let error = CoreError::TooManySources {
            maximum: u32::MAX,
        };

        assert!(error.is_capacity_error());
        assert!(!error.is_source_error());
    }

    #[test]
    fn classifies_source_errors() {
        let error = CoreError::EmptySourceName;

        assert!(error.is_source_error());
        assert!(!error.is_capacity_error());
    }
}