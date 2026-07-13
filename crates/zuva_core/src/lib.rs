//! Shared infrastructure for the Zuva compiler and runtime.
//!
//! `zuva_core` provides the low-level types used throughout the compiler:
//!
//! - source-file storage;
//! - source identifiers;
//! - byte-based source spans;
//! - compiler diagnostics;
//! - interned strings;
//! - symbols;
//! - shared errors.
//!
//! This crate must remain independent of higher-level compiler stages such as
//! parsing, semantic analysis, bytecode generation, and command-line handling.

#![forbid(unsafe_code)]

/// Compiler diagnostic types and severity levels.
pub mod diagnostic;

/// Errors produced by shared compiler infrastructure.
pub mod error;

/// String interning infrastructure.
pub mod interner;

/// Source-file and source-map management.
pub mod source;

/// Source-position and span types.
pub mod span;

/// Compact identifiers for interned names.
pub mod symbol;

pub use diagnostic::{
    Diagnostic, DiagnosticCode, DiagnosticLabel, DiagnosticMessage, Severity,
};

pub use error::{CoreError, CoreResult};

pub use interner::StringInterner;

pub use source::{SourceFile, SourceId, SourceMap};

pub use span::{ByteOffset, Span, Spanned};

pub use symbol::Symbol;