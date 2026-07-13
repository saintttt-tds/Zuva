# Contributing to Zuva

Thank you for contributing to the Zuva Programming Language.

Zuva is an early-stage language, compiler, runtime, and developer-tooling project. Contributions must preserve architectural consistency, correctness, security, testability, and long-term maintainability.

## Project Objectives

Zuva is intended to support:

- systems programming;
- desktop applications;
- mobile applications;
- web applications;
- server and cloud software;
- embedded systems;
- operating-system development;
- artificial intelligence workloads;
- quantum computing;
- hybrid classical and quantum systems.

During the initial development phase, the project is focused on:

- lexical analysis;
- parsing;
- abstract syntax tree construction;
- diagnostics;
- semantic analysis;
- bytecode generation;
- virtual-machine execution;
- command-line tooling;
- standard-library foundations.

## Code of Conduct

Contributors must:

- communicate professionally;
- review technical arguments objectively;
- avoid personal attacks;
- provide constructive feedback;
- respect security and confidentiality requirements;
- avoid intentionally disruptive changes;
- disclose conflicts of interest where relevant.

Harassment, discrimination, threats, deliberate sabotage, and abusive conduct are not accepted.

## Before Contributing

Before starting substantial work:

1. Search existing issues and pull requests.
2. Review the language specification and compiler architecture.
3. Confirm that the work fits the current development phase.
4. Open a design issue for major language or architectural changes.
5. Avoid implementing major syntax changes without prior agreement.

Small fixes, tests, documentation corrections, and isolated internal improvements may be submitted directly.

## Development Environment

Required tools:

- Rust 1.85 or later;
- Cargo;
- Git;
- Rustfmt;
- Clippy;
- PowerShell 7 or a POSIX-compatible shell.

Verify the environment:

```bash
rustc --version
cargo --version
cargo fmt --version
cargo clippy --version
git --version
```

The repository includes `rust-toolchain.toml`, so Rustup should install the required compiler version and components automatically.

## Repository Setup

Fork or clone the repository:

```bash
git clone https://github.com/your-organisation/zuva.git
cd zuva
```

Check the workspace:

```bash
cargo check --workspace
```

Run the tests:

```bash
cargo test --workspace
```

Run the CLI:

```bash
cargo run -p zuva_cli -- --help
```

## Repository Architecture

```text
zuva/
├── crates/
│   ├── zuva_core/
│   ├── zuva_frontend/
│   ├── zuva_semantic/
│   ├── zuva_vm/
│   ├── zuva_driver/
│   └── zuva_cli/
│
├── stdlib/
├── examples/
├── tests/
├── tools/
├── benchmarks/
├── docs/
├── scripts/
└── .github/
```

## Crate Responsibilities

### `zuva_core`

Contains low-level infrastructure shared by compiler components:

- source files;
- source maps;
- spans;
- diagnostics;
- symbols;
- string interning;
- shared errors.

It must not depend on higher-level compiler crates.

### `zuva_frontend`

Contains:

- lexer;
- token definitions;
- parser;
- abstract syntax tree;
- syntax errors.

It may depend on `zuva_core`.

It must not depend on semantic analysis, the VM, the driver, or the CLI.

### `zuva_semantic`

Contains:

- high-level intermediate representation;
- name resolution;
- scopes;
- symbol tables;
- type checking;
- type inference;
- semantic lowering.

It may depend on:

- `zuva_core`;
- `zuva_frontend`.

### `zuva_vm`

Contains:

- bytecode instructions;
- bytecode compiler;
- runtime values;
- call frames;
- heap management;
- garbage collection;
- built-in functions;
- virtual-machine execution.

It may depend on lower-level compiler representations but must not depend on the CLI.

### `zuva_driver`

Coordinates the complete compiler pipeline:

```text
source
  ↓
frontend
  ↓
semantic analysis
  ↓
bytecode generation
  ↓
execution or output
```

The driver owns:

- compilation sessions;
- target configuration;
- pipeline orchestration;
- compiler options;
- compilation results.

### `zuva_cli`

Contains:

- command-line parsing;
- commands;
- REPL;
- terminal output;
- process exit handling.

The CLI should invoke compiler functionality through `zuva_driver`.

It must not duplicate compiler logic.

## Dependency Direction

The expected dependency direction is:

```text
zuva_core
    ↑
zuva_frontend
    ↑
zuva_semantic
    ↑
zuva_vm
    ↑
zuva_driver
    ↑
zuva_cli
```

Rules:

- circular dependencies are prohibited;
- low-level crates must not import higher-level crates;
- the frontend must not know about the VM;
- the semantic layer must not know about the CLI;
- the CLI must not implement parsing or type checking;
- shared infrastructure belongs in `zuva_core`;
- unrelated utilities must not be placed in `zuva_core` merely for convenience.

## Branching

Create a branch from the latest main branch:

```bash
git checkout main
git pull
git checkout -b type/short-description
```

Recommended branch prefixes:

```text
feature/
fix/
docs/
refactor/
test/
security/
performance/
build/
ci/
```

Examples:

```text
feature/add-string-tokens
fix/parser-unclosed-block
docs/type-system-generics
refactor/source-map-storage
test/lexer-unicode-cases
security/validate-bytecode-offsets
performance/reduce-token-allocation
```

## Commit Messages

Use clear, imperative commit messages.

Recommended format:

```text
type(scope): description
```

Examples:

```text
feat(lexer): add hexadecimal integer literals
fix(parser): reject missing function parameter types
docs(grammar): define operator precedence
test(vm): add stack-overflow regression test
refactor(core): simplify source span representation
perf(lexer): reduce identifier allocations
security(vm): validate bytecode jump targets
```

Recommended commit types:

| Type | Purpose |
|---|---|
| `feat` | New functionality |
| `fix` | Bug correction |
| `docs` | Documentation |
| `test` | Tests |
| `refactor` | Internal restructuring |
| `perf` | Performance improvement |
| `security` | Security-related change |
| `build` | Build configuration |
| `ci` | Continuous integration |
| `chore` | Maintenance |

Keep commits focused. Do not combine unrelated changes in one commit.

## Rust Coding Standards

All Rust code must:

- compile without warnings;
- pass Rustfmt;
- pass Clippy;
- avoid unnecessary allocations;
- use explicit error handling;
- avoid unexplained unsafe code;
- include tests for non-trivial behavior;
- preserve dependency boundaries;
- use descriptive names;
- avoid hidden global mutable state.

Format code with:

```bash
cargo fmt --all
```

Check formatting with:

```bash
cargo fmt --all -- --check
```

Run Clippy with:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Naming Conventions

Use Rust naming conventions:

| Item | Convention |
|---|---|
| Crates | `snake_case` |
| Modules | `snake_case` |
| Functions | `snake_case` |
| Variables | `snake_case` |
| Types | `PascalCase` |
| Traits | `PascalCase` |
| Enum variants | `PascalCase` |
| Constants | `SCREAMING_SNAKE_CASE` |

Examples:

```rust
pub struct SourceFile;

pub enum TokenKind {
    Identifier,
    IntegerLiteral,
    EndOfFile,
}

pub fn scan_tokens() {}

pub const MAX_SOURCE_SIZE: usize = 16 * 1024 * 1024;
```

## Error Handling

Do not use `unwrap()` or `expect()` in production compiler code unless an invariant has already been proven and documented.

Avoid:

```rust
let token = tokens.next().unwrap();
```

Prefer:

```rust
let token = tokens
    .next()
    .ok_or_else(|| ParseError::unexpected_end_of_input(span))?;
```

Acceptable uses of `unwrap()` include:

- tests;
- prototypes that are not merged into production paths;
- statically guaranteed internal invariants with an explanatory comment.

Compiler errors must not normally cause process panics.

Malformed Zuva source code must produce diagnostics rather than crash the compiler.

## Diagnostics

Every user-facing compiler diagnostic should include, where applicable:

- error code;
- severity;
- concise message;
- primary source span;
- secondary labels;
- explanatory note;
- corrective suggestion.

Example format:

```text
error[ZP0004]: expected expression
  ┌─ example.zuva:4:16
  │
4 │     let value =
  │                ^ expected an expression after `=`
  │
  = help: provide a value or remove the assignment
```

Do not expose internal Rust implementation details in normal compiler diagnostics.

Avoid user-facing messages such as:

```text
called Option::unwrap() on a None value
```

## Source Spans

All tokens and syntax nodes that can produce diagnostics should retain source-location information.

A span should normally identify:

- source file;
- starting byte offset;
- ending byte offset.

Do not use line and column numbers as the primary internal representation. They should be calculated from byte offsets when diagnostics are rendered.

## Lexer Contributions

Lexer changes must include tests for:

- valid input;
- invalid input;
- end-of-file handling;
- source spans;
- Unicode behavior where relevant;
- adjacent token boundaries;
- malformed literals;
- comments and whitespace.

The lexer must always make progress. It must not enter an infinite loop on malformed input.

## Parser Contributions

Parser changes must include tests for:

- valid syntax;
- invalid syntax;
- precedence;
- associativity;
- source spans;
- error recovery;
- incomplete input;
- nested structures.

The parser should report useful errors and continue where safe, rather than terminating after the first recoverable error.

## AST Contributions

AST types should represent source syntax clearly.

Do not place:

- runtime values;
- bytecode instructions;
- target-specific data;
- CLI configuration;

inside the AST layer.

AST nodes should remain suitable for:

- diagnostics;
- formatting;
- semantic lowering;
- language tooling.

## Semantic Analysis Contributions

Semantic changes should define:

- scope behavior;
- symbol lookup behavior;
- type rules;
- inference rules;
- error conditions;
- source-span ownership;
- recovery behavior.

Changes to the type system must include specification updates.

Type-checking logic should not depend on terminal output or process exit codes.

## Virtual Machine Contributions

Virtual-machine changes must consider:

- stack bounds;
- heap bounds;
- instruction validation;
- invalid bytecode;
- integer overflow;
- object lifetime;
- garbage-collector roots;
- recursive calls;
- runtime diagnostics;
- deterministic execution behavior.

New bytecode instructions must document:

- opcode;
- operands;
- stack input;
- stack output;
- failure behavior;
- encoding;
- validation rules.

Example:

```text
ADD_INT

Operands:
    None

Stack before:
    [..., left: Int, right: Int]

Stack after:
    [..., result: Int]

Errors:
    TypeMismatch
    IntegerOverflow
```

## Unsafe Rust

Unsafe Rust is permitted only when required for:

- foreign-function interfaces;
- operating-system integration;
- low-level memory management;
- optimized runtime internals;
- hardware interaction.

Every unsafe block must include a `SAFETY` comment.

Example:

```rust
// SAFETY:
// `pointer` is non-null, correctly aligned, points to initialized memory,
// and remains valid for the duration of this read.
let value = unsafe { pointer.read() };
```

Unsafe abstractions must expose a safe public interface wherever practical.

A pull request containing unsafe code requires additional review.

## Testing Requirements

Every behavioral change should include tests.

Run:

```bash
cargo test --workspace
```

Test categories:

```text
tests/
├── compile-pass/
├── compile-fail/
├── runtime/
├── end-to-end/
└── snapshots/
```

### Compile-pass tests

These contain valid Zuva programs that must compile successfully.

### Compile-fail tests

These contain invalid programs that must produce defined diagnostics.

Each compile-fail test should document the expected error.

### Runtime tests

These verify execution results and runtime failures.

### End-to-end tests

These verify the complete toolchain:

```text
source file
   ↓
CLI
   ↓
compiler
   ↓
VM
   ↓
process output
```

### Regression tests

Every confirmed bug fix should include a regression test that fails before the fix and passes afterward.

## Snapshot Tests

Snapshot tests may be used for:

- diagnostics;
- formatted syntax;
- AST output;
- HIR output;
- bytecode disassembly;
- CLI output.

Review snapshot changes carefully. Do not accept snapshot updates automatically without checking their correctness.

## Benchmarks

Performance-sensitive changes should include benchmark evidence where practical.

Run benchmarks with:

```bash
cargo bench -p zuva_benchmarks
```

Relevant benchmark areas include:

- tokenization speed;
- parser throughput;
- memory usage;
- semantic-analysis time;
- bytecode compilation;
- VM instruction dispatch;
- garbage collection;
- startup time.

Do not merge a performance optimization that substantially reduces readability or safety without documented evidence of its benefit.

## Documentation Requirements

Update documentation when changing:

- syntax;
- grammar;
- type behavior;
- compiler architecture;
- command-line behavior;
- bytecode format;
- runtime semantics;
- public Rust APIs;
- package formats;
- security behavior.

Language changes should normally update files under:

```text
docs/language/
```

Compiler architecture changes should update:

```text
docs/compiler/
```

Major architectural decisions should include an Architecture Decision Record under:

```text
docs/decisions/
```

## Architecture Decision Records

Use an ADR for decisions such as:

- parser architecture;
- garbage-collection strategy;
- ownership model;
- type-inference model;
- bytecode encoding;
- native-code backend;
- package-resolution algorithm;
- standard-library compatibility policy.

ADR format:

```markdown
# ADR-NNN: Decision Title

## Status

Proposed | Accepted | Rejected | Superseded

## Context

Describe the problem and constraints.

## Decision

Describe the chosen solution.

## Consequences

Describe advantages, disadvantages, risks, and future implications.

## Alternatives Considered

Describe rejected options and the reasons for rejection.
```

## Language Design Changes

A language design proposal must include:

- motivation;
- syntax;
- semantics;
- type-system interaction;
- memory-model interaction;
- examples;
- invalid examples;
- diagnostics;
- compatibility impact;
- implementation outline;
- alternatives;
- unresolved questions.

Do not change the language solely because another programming language uses a particular feature.

Each feature must fit Zuva's goals and remain internally coherent.

## Backward Compatibility

Before the first stable release, breaking changes are permitted but must be documented.

After the stable language specification is established:

- syntax should remain backward-compatible where practical;
- deprecations should precede removals;
- migration guidance should be provided;
- compiler diagnostics should identify deprecated behavior;
- serialized formats should use explicit versioning.

## Security Requirements

Security-sensitive changes must follow `SECURITY.md`.

Do not publish exploit details for unresolved vulnerabilities.

Do not commit:

- credentials;
- API keys;
- signing keys;
- private certificates;
- tokens;
- passwords;
- production configuration;
- personal data.

Run relevant dependency checks:

```bash
cargo audit
cargo deny check
```

## Dependency Policy

New dependencies require justification.

Before adding a crate, assess:

- whether the functionality can reasonably be implemented internally;
- maintenance activity;
- release history;
- license;
- known vulnerabilities;
- transitive dependency count;
- platform compatibility;
- compile-time cost;
- binary-size cost;
- long-term stability.

Avoid adding dependencies for trivial functionality.

Workspace dependencies should generally be declared in the root `Cargo.toml`.

Example:

```toml
[workspace.dependencies]
thiserror = "2"
```

Crates should reference them with:

```toml
[dependencies]
thiserror.workspace = true
```

## Standard Library Contributions

Standard-library APIs should prioritize:

- consistency;
- safety;
- portability;
- predictable performance;
- minimal hidden allocation;
- clear failure behavior;
- long-term compatibility.

Standard-library additions require:

- API documentation;
- usage examples;
- tests;
- platform analysis;
- error semantics;
- performance considerations.

Target-specific behavior must be documented explicitly.

## Pull Request Requirements

A pull request should include:

- a clear title;
- a concise summary;
- motivation;
- implementation description;
- tests performed;
- documentation changes;
- compatibility impact;
- security impact;
- performance impact;
- unresolved limitations.

Suggested pull request description:

```markdown
## Summary

Describe the change.

## Motivation

Explain why it is necessary.

## Implementation

Describe the technical approach.

## Testing

List tests and commands executed.

## Compatibility

Describe syntax, API, bytecode, or behavioral compatibility effects.

## Security

Describe security implications.

## Performance

Describe expected performance impact.

## Documentation

List documentation updated.
```

## Required Checks

Before submitting a pull request, run:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

When applicable, also run:

```bash
cargo bench -p zuva_benchmarks
cargo audit
cargo deny check
```

## Review Requirements

At least one maintainer approval is required for ordinary changes.

Additional review is required for changes involving:

- unsafe Rust;
- language syntax;
- type-system behavior;
- memory management;
- bytecode format;
- package security;
- cryptography;
- operating-system support;
- quantum compilation;
- release infrastructure.

Reviewers should evaluate:

- correctness;
- architecture;
- diagnostics;
- security;
- performance;
- tests;
- maintainability;
- compatibility;
- documentation.

## Generated Files

Do not manually edit generated files unless the generation system explicitly requires it.

Generated files should clearly indicate:

```text
This file is generated. Do not edit manually.
```

The following should not be committed unless specifically required:

```text
target/
coverage/
temporary files
local configuration
compiler caches
release archives
```

## Issue Reporting

Bug reports should include:

- Zuva version or commit;
- operating system;
- hardware architecture;
- Rust version;
- command executed;
- source code reproducer;
- expected behavior;
- actual behavior;
- compiler output;
- stack trace, when available.

Use minimal reproducible examples.

Do not include confidential source code or credentials.

## Feature Requests

Feature requests should explain:

- the problem;
- the proposed behavior;
- use cases;
- alternatives;
- compatibility concerns;
- implementation complexity;
- why the feature belongs in Zuva or its official ecosystem.

## Release Policy

Only authorized maintainers may publish official releases.

Release preparation should include:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo audit
cargo deny check
```

Official releases should use:

- signed Git tags;
- checksums;
- signed artifacts;
- reproducible builds where practical;
- release notes;
- versioned documentation;
- controlled publishing credentials.

## Licensing

By submitting a contribution, you agree that your contribution may be distributed under the Apache License 2.0 used by this project.

Contributors must not submit code they do not have the right to license.

Third-party code must include appropriate attribution and compatible licensing information.

## Contribution Checklist

Before submission, confirm:

- [ ] The change has a clear purpose.
- [ ] The architecture remains consistent.
- [ ] Dependency direction is preserved.
- [ ] Rust code is formatted.
- [ ] Clippy reports no warnings.
- [ ] All tests pass.
- [ ] New behavior has tests.
- [ ] Error cases are tested.
- [ ] Diagnostics are readable.
- [ ] Documentation is updated.
- [ ] Unsafe code is justified.
- [ ] Security implications are considered.
- [ ] Performance implications are considered.
- [ ] No secrets or generated build files are committed.
- [ ] Commit messages are clear.
- [ ] The pull request description is complete.