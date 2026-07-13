//! Compiler diagnostics and source labels.
//!
//! Diagnostics describe errors, warnings, advice, and informational messages
//! produced by the Zuva compiler.
//!
//! This module stores diagnostic information independently from terminal
//! rendering. A later reporting layer will convert these structures into
//! formatted command-line, IDE, JSON, or machine-readable output.

use std::fmt;

use smol_str::SmolStr;

use crate::span::Span;

/// Severity assigned to a compiler diagnostic.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Severity {
    /// An internal compiler failure or violated invariant.
    Bug,

    /// A compilation error that prevents successful output.
    Error,

    /// A suspicious construct that does not necessarily prevent compilation.
    Warning,

    /// A recommendation that may improve the source code.
    Advice,

    /// Additional informational output.
    Note,
}

impl Severity {
    /// Returns the human-readable name of the severity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bug => "bug",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Advice => "advice",
            Self::Note => "note",
        }
    }

    /// Returns `true` when the diagnostic should cause compilation to fail.
    #[must_use]
    pub const fn is_fatal(self) -> bool {
        matches!(self, Self::Bug | Self::Error)
    }

    /// Returns `true` when the diagnostic is an error.
    #[must_use]
    pub const fn is_error(self) -> bool {
        matches!(self, Self::Error)
    }

    /// Returns `true` when the diagnostic is an internal compiler bug.
    #[must_use]
    pub const fn is_bug(self) -> bool {
        matches!(self, Self::Bug)
    }

    /// Returns `true` when the diagnostic is a warning.
    #[must_use]
    pub const fn is_warning(self) -> bool {
        matches!(self, Self::Warning)
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Stable identifier assigned to a category of diagnostic.
///
/// Diagnostic codes should remain stable once published because external
/// tooling, tests, documentation, and users may depend on them.
///
/// Example codes:
///
/// ```text
/// ZL0001
/// ZP0004
/// ZT0012
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticCode(SmolStr);

impl DiagnosticCode {
    /// Creates a diagnostic code.
    #[must_use]
    pub fn new(code: impl Into<SmolStr>) -> Self {
        Self(code.into())
    }

    /// Returns the diagnostic code as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the code and returns its stored string.
    #[must_use]
    pub fn into_inner(self) -> SmolStr {
        self.0
    }
}

impl AsRef<str> for DiagnosticCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for DiagnosticCode {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticCode {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SmolStr> for DiagnosticCode {
    fn from(value: SmolStr) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// Text attached to a compiler diagnostic.
///
/// A dedicated type prevents diagnostic text from being confused with source
/// code, symbols, file names, or other compiler strings.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DiagnosticMessage(SmolStr);

impl DiagnosticMessage {
    /// Creates a diagnostic message.
    #[must_use]
    pub fn new(message: impl Into<SmolStr>) -> Self {
        Self(message.into())
    }

    /// Returns the message as text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns `true` when the message is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the message length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Consumes the message and returns its stored string.
    #[must_use]
    pub fn into_inner(self) -> SmolStr {
        self.0
    }
}

impl AsRef<str> for DiagnosticMessage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<&str> for DiagnosticMessage {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for DiagnosticMessage {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<SmolStr> for DiagnosticMessage {
    fn from(value: SmolStr) -> Self {
        Self::new(value)
    }
}

impl fmt::Display for DiagnosticMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// A source span highlighted by a compiler diagnostic.
///
/// Labels may be primary or secondary:
///
/// - a primary label identifies the main source of the diagnostic;
/// - a secondary label provides related context.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticLabel {
    span: Span,
    message: Option<DiagnosticMessage>,
    primary: bool,
}

impl DiagnosticLabel {
    /// Creates a primary source label.
    #[must_use]
    pub const fn primary(span: Span) -> Self {
        Self {
            span,
            message: None,
            primary: true,
        }
    }

    /// Creates a secondary source label.
    #[must_use]
    pub const fn secondary(span: Span) -> Self {
        Self {
            span,
            message: None,
            primary: false,
        }
    }

    /// Creates a primary source label containing explanatory text.
    #[must_use]
    pub fn primary_with_message(
        span: Span,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self {
            span,
            message: Some(message.into()),
            primary: true,
        }
    }

    /// Creates a secondary source label containing explanatory text.
    #[must_use]
    pub fn secondary_with_message(
        span: Span,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self {
            span,
            message: Some(message.into()),
            primary: false,
        }
    }

    /// Returns the source span associated with this label.
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the label's optional explanatory message.
    #[must_use]
    pub fn message(&self) -> Option<&DiagnosticMessage> {
        self.message.as_ref()
    }

    /// Returns `true` when this is the primary label.
    #[must_use]
    pub const fn is_primary(&self) -> bool {
        self.primary
    }

    /// Returns `true` when this is a secondary label.
    #[must_use]
    pub const fn is_secondary(&self) -> bool {
        !self.primary
    }

    /// Attaches an explanatory message to the label.
    #[must_use]
    pub fn with_message(
        mut self,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        self.message = Some(message.into());
        self
    }
}

/// Structured compiler diagnostic.
///
/// A diagnostic contains:
///
/// - a stable code;
/// - severity;
/// - primary message;
/// - zero or more source labels;
/// - zero or more explanatory notes;
/// - an optional corrective suggestion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: DiagnosticMessage,
    labels: Vec<DiagnosticLabel>,
    notes: Vec<DiagnosticMessage>,
    help: Option<DiagnosticMessage>,
}

impl Diagnostic {
    /// Creates a diagnostic.
    #[must_use]
    pub fn new(
        code: impl Into<DiagnosticCode>,
        severity: Severity,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self {
            code: code.into(),
            severity,
            message: message.into(),
            labels: Vec::new(),
            notes: Vec::new(),
            help: None,
        }
    }

    /// Creates an error diagnostic.
    #[must_use]
    pub fn error(
        code: impl Into<DiagnosticCode>,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self::new(code, Severity::Error, message)
    }

    /// Creates a warning diagnostic.
    #[must_use]
    pub fn warning(
        code: impl Into<DiagnosticCode>,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self::new(code, Severity::Warning, message)
    }

    /// Creates an internal compiler bug diagnostic.
    #[must_use]
    pub fn bug(
        code: impl Into<DiagnosticCode>,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self::new(code, Severity::Bug, message)
    }

    /// Creates an advice diagnostic.
    #[must_use]
    pub fn advice(
        code: impl Into<DiagnosticCode>,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self::new(code, Severity::Advice, message)
    }

    /// Creates an informational note diagnostic.
    #[must_use]
    pub fn note(
        code: impl Into<DiagnosticCode>,
        message: impl Into<DiagnosticMessage>,
    ) -> Self {
        Self::new(code, Severity::Note, message)
    }

    /// Returns the stable diagnostic code.
    #[must_use]
    pub const fn code(&self) -> &DiagnosticCode {
        &self.code
    }

    /// Returns the diagnostic severity.
    #[must_use]
    pub const fn severity(&self) -> Severity {
        self.severity
    }

    /// Returns the primary diagnostic message.
    #[must_use]
    pub const fn message(&self) -> &DiagnosticMessage {
        &self.message
    }

    /// Returns all source labels.
    #[must_use]
    pub fn labels(&self) -> &[DiagnosticLabel] {
        &self.labels
    }

    /// Returns all explanatory notes.
    #[must_use]
    pub fn notes(&self) -> &[DiagnosticMessage] {
        &self.notes
    }

    /// Returns the optional corrective suggestion.
    #[must_use]
    pub fn help(&self) -> Option<&DiagnosticMessage> {
        self.help.as_ref()
    }

    /// Returns `true` when this diagnostic prevents successful compilation.
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        self.severity.is_fatal()
    }

    /// Adds a source label.
    pub fn push_label(&mut self, label: DiagnosticLabel) {
        self.labels.push(label);
    }

    /// Adds an explanatory note.
    pub fn push_note(&mut self, note: impl Into<DiagnosticMessage>) {
        self.notes.push(note.into());
    }

    /// Sets the corrective suggestion.
    pub fn set_help(&mut self, help: impl Into<DiagnosticMessage>) {
        self.help = Some(help.into());
    }

    /// Adds a source label using builder-style syntax.
    #[must_use]
    pub fn with_label(mut self, label: DiagnosticLabel) -> Self {
        self.push_label(label);
        self
    }

    /// Adds an explanatory note using builder-style syntax.
    #[must_use]
    pub fn with_note(
        mut self,
        note: impl Into<DiagnosticMessage>,
    ) -> Self {
        self.push_note(note);
        self
    }

    /// Adds a corrective suggestion using builder-style syntax.
    #[must_use]
    pub fn with_help(
        mut self,
        help: impl Into<DiagnosticMessage>,
    ) -> Self {
        self.set_help(help);
        self
    }

    /// Returns the primary source label, when one exists.
    #[must_use]
    pub fn primary_label(&self) -> Option<&DiagnosticLabel> {
        self.labels.iter().find(|label| label.is_primary())
    }

    /// Returns an iterator over secondary source labels.
    pub fn secondary_labels(
        &self,
    ) -> impl Iterator<Item = &DiagnosticLabel> {
        self.labels.iter().filter(|label| label.is_secondary())
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}[{}]: {}",
            self.severity,
            self.code,
            self.message
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        source::SourceId,
        span::ByteOffset,
    };

    use super::*;

    fn test_span(start: u32, end: u32) -> Span {
        Span::new(
            SourceId::from_raw(0),
            ByteOffset::new(start),
            ByteOffset::new(end),
        )
    }

    #[test]
    fn creates_error_diagnostic() {
        let diagnostic =
            Diagnostic::error("ZP0001", "expected an expression");

        assert_eq!(diagnostic.code().as_str(), "ZP0001");
        assert_eq!(diagnostic.severity(), Severity::Error);
        assert_eq!(
            diagnostic.message().as_str(),
            "expected an expression"
        );
        assert!(diagnostic.is_fatal());
    }

    #[test]
    fn creates_warning_diagnostic() {
        let diagnostic =
            Diagnostic::warning("ZW0001", "unused variable");

        assert_eq!(diagnostic.severity(), Severity::Warning);
        assert!(!diagnostic.is_fatal());
    }

    #[test]
    fn adds_primary_label() {
        let span = test_span(4, 9);

        let diagnostic =
            Diagnostic::error("ZP0002", "invalid assignment")
                .with_label(
                    DiagnosticLabel::primary_with_message(
                        span,
                        "assignment occurs here",
                    ),
                );

        let label = diagnostic
            .primary_label()
            .expect("a primary label should exist");

        assert_eq!(label.span(), span);
        assert_eq!(
            label.message().map(DiagnosticMessage::as_str),
            Some("assignment occurs here")
        );
    }

    #[test]
    fn stores_notes_and_help() {
        let diagnostic =
            Diagnostic::error("ZT0001", "type mismatch")
                .with_note("expected `Int` but found `Text`")
                .with_help("convert the text value to an integer");

        assert_eq!(diagnostic.notes().len(), 1);
        assert_eq!(
            diagnostic.help().map(DiagnosticMessage::as_str),
            Some("convert the text value to an integer")
        );
    }

    #[test]
    fn separates_primary_and_secondary_labels() {
        let primary = test_span(0, 4);
        let secondary = test_span(10, 14);

        let diagnostic =
            Diagnostic::error("ZN0001", "duplicate declaration")
                .with_label(DiagnosticLabel::primary(primary))
                .with_label(DiagnosticLabel::secondary(secondary));

        assert_eq!(
            diagnostic
                .primary_label()
                .expect("a primary label should exist")
                .span(),
            primary
        );

        let secondary_labels =
            diagnostic.secondary_labels().collect::<Vec<_>>();

        assert_eq!(secondary_labels.len(), 1);
        assert_eq!(secondary_labels[0].span(), secondary);
    }

    #[test]
    fn formats_diagnostic_summary() {
        let diagnostic =
            Diagnostic::error("ZP0004", "expected expression");

        assert_eq!(
            diagnostic.to_string(),
            "error[ZP0004]: expected expression"
        );
    }

    #[test]
    fn classifies_fatal_severities() {
        assert!(Severity::Bug.is_fatal());
        assert!(Severity::Error.is_fatal());
        assert!(!Severity::Warning.is_fatal());
        assert!(!Severity::Advice.is_fatal());
        assert!(!Severity::Note.is_fatal());
    }
}