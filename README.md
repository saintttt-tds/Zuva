# Zuva Programming Language

Zuva is a compiled, statically typed, multi-paradigm programming language intended for systems programming, application development, web services, cloud infrastructure, mobile applications, artificial intelligence, embedded systems, operating systems, and future quantum and hybrid computing environments.

The initial compiler implementation is written in Rust. A later objective is for the Zuva compiler to become self-hosting.

## Project Status

Zuva is currently in the bootstrap and language-foundation phase.

Current development priorities:

- source-code management;
- lexical analysis;
- parsing;
- abstract syntax tree construction;
- diagnostics;
- semantic analysis;
- bytecode generation;
- virtual-machine execution;
- command-line tooling;
- standard-library foundations.

Zuva is not yet production-ready.

## Design Principles

Zuva is designed around five principles:

1. **Performance**  
   Produce efficient native machine code with predictable execution characteristics.

2. **Safety**  
   Prevent common programming errors through static typing, controlled memory access, and secure defaults.

3. **Productivity**  
   Provide readable syntax, integrated tooling, useful diagnostics, and comprehensive standard libraries.

4. **Scalability**  
   Support small programs, enterprise services, distributed systems, and operating-system components.

5. **Longevity**  
   Maintain an extensible architecture capable of supporting future hardware and computing paradigms.

## Planned Language Capabilities

Zuva is intended to support:

- procedural programming;
- functional programming;
- object-oriented programming;
- generic programming;
- pattern matching;
- algebraic data types;
- asynchronous programming;
- structured concurrency;
- ownership and borrowing;
- controlled unsafe operations;
- compile-time metaprogramming;
- native interoperability;
- package-based modular development.

## Planned Compilation Targets

The long-term target matrix includes:

- Linux;
- Windows;
- macOS;
- Android;
- iOS;
- WebAssembly;
- x86-64;
- ARM64;
- RISC-V;
- bare-metal embedded systems;
- operating-system kernels;
- quantum intermediate representations;
- hybrid CPU, GPU, NPU, FPGA, and QPU systems.

## Repository Architecture

```text
zuva/
├── crates/
│   ├── zuva_core/       # Shared source, span, symbol, and diagnostic infrastructure
│   ├── zuva_frontend/   # Lexer, parser, and abstract syntax tree
│   ├── zuva_semantic/   # Name resolution, HIR, and type checking
│   ├── zuva_vm/         # Bytecode compiler and virtual machine
│   ├── zuva_driver/     # Compiler pipeline orchestration
│   └── zuva_cli/        # Command-line interface and REPL
│
├── stdlib/               # Zuva standard library
├── examples/             # Example Zuva programs
├── tests/                # Compiler and runtime tests
├── tools/xtask/          # Repository automation
├── benchmarks/           # Compiler and runtime benchmarks
├── docs/                 # Language and implementation documentation
├── scripts/              # Development scripts
└── .github/              # CI, security, and release automation
```

## Compiler Pipeline

```text
Zuva source code
        │
        ▼
Lexer
        │
        ▼
Parser
        │
        ▼
Abstract Syntax Tree
        │
        ▼
Semantic Analysis
        ├── Name resolution
        ├── Type checking
        └── HIR lowering
        │
        ▼
Bytecode Compiler
        │
        ▼
Zuva Virtual Machine
```

Future compiler stages will add:

```text
HIR
 │
 ▼
MIR
 │
 ▼
Optimization
 │
 ├── LLVM backend ─────► Native binaries
 ├── WebAssembly ──────► Web applications
 ├── QIR backend ──────► Quantum programs
 └── Hybrid backend ───► Heterogeneous systems
```

## Prerequisites

Install:

- Rust 1.85 or later;
- Cargo;
- Git;
- PowerShell 7 or a POSIX-compatible shell.

The repository includes `rust-toolchain.toml`, which allows Rustup to install the required toolchain automatically.

Verify the installation:

```bash
rustc --version
cargo --version
git --version
```

## Building the Workspace

Clone the repository:

```bash
git clone https://github.com/your-organisation/zuva.git
cd zuva
```

Check the complete workspace:

```bash
cargo check --workspace
```

Build all crates:

```bash
cargo build --workspace
```

Build an optimized release:

```bash
cargo build --workspace --release
```

## Running the CLI

Display help:

```bash
cargo run -p zuva_cli -- --help
```

Run a Zuva source file:

```bash
cargo run -p zuva_cli -- run examples/basics/hello_world.zuva
```

Check a source file without executing it:

```bash
cargo run -p zuva_cli -- check examples/basics/functions.zuva
```

Start the interactive REPL:

```bash
cargo run -p zuva_cli -- repl
```

After installation, the intended command format is:

```bash
zuva run program.zuva
zuva check program.zuva
zuva build program.zuva
zuva repl
zuva --version
```

## Example Zuva Program

```zuva
fn fibonacci(n: Int) -> Int {
    if n <= 1 {
        return n
    }

    return fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let result = fibonacci(10)
    print(result)
}
```

## Testing

Run all tests:

```bash
cargo test --workspace
```

Run a specific crate:

```bash
cargo test -p zuva_frontend
```

Run documentation tests:

```bash
cargo test --workspace --doc
```

The test suites are organized as follows:

```text
tests/
├── compile-pass/   # Valid programs that must compile
├── compile-fail/   # Invalid programs that must report expected errors
├── runtime/        # Programs with expected execution results
├── end-to-end/     # CLI and complete pipeline tests
└── snapshots/      # Diagnostic and output snapshots
```

## Formatting

Check formatting:

```bash
cargo fmt --all -- --check
```

Apply formatting:

```bash
cargo fmt --all
```

## Linting

Run Clippy:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Benchmarks

Run all benchmarks:

```bash
cargo bench -p zuva_benchmarks
```

Benchmark groups include:

- lexer performance;
- parser performance;
- bytecode compilation;
- virtual-machine execution.

## Documentation

The primary documentation directories are:

```text
docs/
├── language/       # Syntax, grammar, types, modules, and memory model
├── compiler/       # Compiler and VM architecture
├── development/    # Build, test, debug, and release processes
└── decisions/      # Architecture Decision Records
```

Generate Rust API documentation:

```bash
cargo doc --workspace --no-deps
```

Open generated documentation:

```bash
cargo doc --workspace --no-deps --open
```

## Dependency Direction

The compiler follows a strict low-to-high dependency structure:

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

- low-level crates must not depend on higher-level crates;
- `zuva_core` must remain independent of compiler stages;
- the frontend must not depend on the virtual machine;
- the semantic layer must not depend on the CLI;
- the CLI must access compiler functionality through `zuva_driver`;
- circular crate dependencies are prohibited.

## Security

Security issues must not be submitted through public issue trackers.

See [`SECURITY.md`](SECURITY.md) for the vulnerability-reporting process.

The project intends to implement:

- memory-safe defaults;
- explicit unsafe boundaries;
- source and package verification;
- capability-aware APIs;
- dependency auditing;
- reproducible builds;
- cryptographic release signing;
- controlled foreign-function interfaces.

## Contributing

Contribution requirements are documented in [`CONTRIBUTING.md`](CONTRIBUTING.md).

Before submitting code, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Versioning

Zuva follows semantic versioning for compiler and tooling releases.

During the `0.x` development period, language syntax and internal APIs may change between minor versions.

Stable language releases will prioritize backward compatibility.

## License

Zuva is intended to be distributed under the Apache License 2.0.

See [`LICENSE`](LICENSE) for the complete license terms.

## Project Notice

This repository contains an experimental compiler and programming-language implementation. Interfaces, syntax, runtime behavior, and file formats may change until the language reaches its first stable release.