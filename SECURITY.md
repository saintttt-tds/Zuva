# Security Policy

Security is a core requirement of the Zuva Programming Language, compiler, virtual machine, package system, standard library, and associated developer tools.

This document defines how security vulnerabilities should be reported, assessed, resolved, and disclosed.

## Supported Versions

Zuva is currently under active development and has not reached a stable production release.

| Version | Supported |
|---|---|
| Latest development branch | Yes |
| Latest tagged pre-release | Yes |
| Older pre-release versions | No |
| Unofficial forks | No |

Security fixes will normally be applied to the latest supported development version.

Backward-porting fixes to older versions is not guaranteed during the `0.x` development period.

## Reporting a Vulnerability

Do not report security vulnerabilities through public GitHub issues, discussions, pull requests, or public communication channels.

Report vulnerabilities privately using one of the following methods:

1. GitHub private security advisories, when enabled for the repository.
2. The official Zuva security email address.
3. A designated private communication channel approved by the project maintainers.

Security contact:

zimsecexammate@gmail.com

A vulnerability report should include:

* a clear description of the issue;
* the affected component or crate;
* the affected Zuva version or commit;
* reproduction steps;
* proof-of-concept code, when appropriate;
* expected and actual behavior;
* potential security impact;
* known workarounds;
* the reporter's preferred contact method;
* whether public disclosure has already occurred.

Do not include sensitive user information, private credentials, production secrets, or unrelated confidential data.

## Security Scope

Security reports may cover any official Zuva component, including:

* the compiler;
* lexer and parser;
* semantic analysis;
* type checker;
* ownership and effect systems;
* bytecode compiler;
* virtual machine;
* garbage collector;
* native-code backends;
* WebAssembly backend;
* quantum backend;
* command-line interface;
* REPL;
* package manager;
* build system;
* standard library;
* language server;
* formatter;
* linter;
* debugger;
* installer;
* release pipeline;
* official editor extensions;
* official mobile, desktop, web, and operating-system frameworks.

## Examples of Security Vulnerabilities

Relevant vulnerability classes include:

* arbitrary code execution;
* compiler memory corruption;
* virtual-machine memory corruption;
* sandbox escape;
* unsafe bytecode validation;
* malicious package execution;
* dependency confusion;
* package signature bypass;
* path traversal;
* command injection;
* privilege escalation;
* incorrect capability enforcement;
* unauthorized file access;
* unsafe foreign-function interface behavior;
* denial of service caused by crafted source code;
* uncontrolled compiler resource consumption;
* malformed input causing compiler crashes;
* incorrect cryptographic implementation;
* information disclosure;
* insecure temporary-file handling;
* unsafe deserialization;
* build-script privilege abuse;
* release artifact tampering;
* reproducible-build failures that create supply-chain risk.

## Out-of-Scope Reports

The following are generally outside the security scope:

* unsupported Zuva versions;
* unofficial forks;
* third-party packages not maintained by the Zuva project;
* vulnerabilities requiring unsupported local modifications;
* social engineering;
* physical attacks;
* denial-of-service reports without a reproducible technical cause;
* reports generated entirely by automated scanners without validation;
* missing security headers on non-production project pages;
* general feature requests;
* theoretical issues without a credible attack path.

A report initially considered out of scope may still be reviewed when it identifies a broader architectural weakness.

## Response Process

After receiving a valid report, the Zuva security team should:

1. acknowledge receipt;
2. verify the vulnerability;
3. classify its severity;
4. identify affected versions and components;
5. assign responsible maintainers;
6. develop and review a fix;
7. create regression tests;
8. prepare release notes and advisories;
9. publish patched versions;
10. coordinate public disclosure with the reporter.

Target response times:

| Action                  | Target                          |
| ----------------------- | ------------------------------- |
| Initial acknowledgement | Within 3 business days          |
| Preliminary assessment  | Within 7 business days          |
| Severity classification | Within 10 business days         |
| Remediation plan        | Within 14 business days         |
| Critical fix            | As soon as technically possible |
| Coordinated disclosure  | After a fix is available        |

These are operational targets rather than guarantees.

Complex compiler, runtime, operating-system, or supply-chain vulnerabilities may require additional investigation.

## Severity Classification

Zuva security issues should be classified as follows:

### Critical

A vulnerability that may enable:

* arbitrary code execution;
* remote compromise;
* package-signing bypass;
* compiler or runtime sandbox escape;
* privilege escalation;
* widespread supply-chain compromise;
* silent generation of insecure machine code.

### High

A vulnerability that may enable:

* unauthorized access to sensitive data;
* significant capability bypass;
* reliable denial of service;
* malicious package execution under common configurations;
* major memory-safety violations;
* incorrect security-sensitive compilation.

### Medium

A vulnerability with limited impact, constrained prerequisites, or reduced exploitability.

Examples include:

* local information disclosure;
* restricted denial of service;
* partial permission bypass;
* insecure behavior requiring uncommon configuration.

### Low

A vulnerability with limited security impact or substantial exploitation constraints.

## Coordinated Disclosure

Reporters are requested to avoid public disclosure until:

* the vulnerability has been verified;
* a fix has been developed;
* supported releases have been patched;
* users have had reasonable time to update.

The Zuva project will attempt to coordinate the disclosure date with the reporter.

Security advisories may include:

* affected versions;
* severity;
* technical description;
* impact;
* mitigation;
* patched versions;
* acknowledgement of the reporter;
* relevant security identifiers.

Reporter acknowledgement will be included unless anonymity is requested.

## Safe Harbor

The Zuva project supports good-faith security research.

Research is considered good faith when it:

* avoids unnecessary damage;
* avoids privacy violations;
* avoids disruption of services;
* does not access unrelated data;
* does not alter or destroy data;
* uses only the minimum access required to demonstrate the issue;
* reports findings privately;
* allows reasonable time for remediation;
* complies with applicable law.

The project will not pursue action against researchers who comply with this policy and act in good faith.

This safe-harbor statement does not authorize testing against third-party systems, production infrastructure, or services not owned by the Zuva project.

## Security Requirements for Contributions

Security-sensitive changes must include:

* a written description of the security assumptions;
* tests covering expected and rejected behavior;
* validation of untrusted input;
* explicit handling of failure cases;
* documentation of unsafe Rust code;
* justification for new privileged operations;
* review of dependency changes;
* consideration of denial-of-service risks;
* consideration of cross-platform behavior.

All unsafe Rust blocks must include a safety comment explaining the required invariants.

Example:

```rust
// SAFETY:
// The pointer is non-null, aligned, and references initialized memory
// that remains valid for the duration of this operation.
unsafe {
    perform_low_level_operation();
}
```

## Dependency Security

New dependencies must be evaluated for:

* maintenance status;
* license compatibility;
* known vulnerabilities;
* transitive dependencies;
* release frequency;
* security history;
* necessity;
* platform support.

Recommended security checks include:

```bash
cargo audit
cargo deny check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Dependency versions should be pinned or constrained appropriately for reproducible builds.

## Release Security

Official releases should use:

* protected branches;
* mandatory code review;
* automated tests;
* dependency auditing;
* signed Git tags;
* signed release artifacts;
* checksums;
* reproducible-build verification;
* restricted publishing credentials;
* multi-factor authentication;
* minimal release permissions.

Release credentials must never be committed to the repository.

## Secrets Management

The repository must not contain:

* passwords;
* API keys;
* private cryptographic keys;
* authentication tokens;
* production certificates;
* database credentials;
* cloud-provider credentials;
* signing credentials;
* personal access tokens.

Local secrets must be stored outside version control.

Files containing local secrets should be covered by `.gitignore`.

Example:

```text
.env
.env.local
secrets/
*.key
*.pem
```

## Compiler Security Principles

The Zuva compiler must treat all source code as untrusted input.

Compiler components should:

* reject malformed input safely;
* avoid uncontrolled recursion;
* enforce allocation limits where practical;
* prevent integer overflows in size calculations;
* validate intermediate representations;
* validate generated bytecode;
* isolate unsafe code;
* produce deterministic diagnostics;
* avoid executing source code during ordinary parsing or checking;
* prevent build scripts from receiving unnecessary privileges.

## Runtime Security Principles

The Zuva runtime and virtual machine should:

* validate bytecode before execution;
* enforce stack and heap bounds;
* prevent invalid memory access;
* enforce capability restrictions;
* detect malformed instructions;
* isolate native extensions;
* limit untrusted resource consumption;
* provide deterministic failure behavior;
* avoid exposing host resources by default.

## Package Security Principles

The future Zuva package manager should support:

* cryptographic package signatures;
* package checksums;
* trusted registries;
* dependency-lock files;
* namespace protection;
* package ownership verification;
* reproducible dependency resolution;
* vulnerability advisories;
* package revocation;
* permission declarations;
* restricted build-script execution.

## Security Updates

Security releases may be published outside the normal release schedule.

Users should upgrade immediately when a security advisory affects their environment.

Security advisories will identify:

* affected versions;
* patched versions;
* mitigation steps;
* severity;
* relevant components;
* upgrade instructions.

## Policy Changes

This policy may change as the Zuva language and ecosystem mature.

Material changes should be documented in the repository history and release documentation.

