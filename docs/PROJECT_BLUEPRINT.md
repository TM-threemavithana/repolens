# RepoLens
## Complete Rust Learning-by-Building Project Blueprint

**Project classification**: Educational, portfolio-grade developer tool  
**Primary language**: Rust (Rust 2024 edition; stable toolchain)  
**Development environment**: Google Antigravity  
**Document version**: 1.0 | Prepared: 11 October 2026  
**Status**: Approved learning direction; proposed specifications to validate incrementally  
**Relationship to other work**: Independent from the Scala-based DevForge project.  
**Project mission**: Learn Rust from beginner through advanced by personally understanding, implementing, testing, and evolving a useful repository analysis tool. The learning process is a first-class requirement; automated code production is not the goal.

---

### 1. Executive Summary
RepoLens is a cross-platform developer tool that analyzes source repositories, summarizes project structure, checks configurable quality rules, and generates machine-readable and human-readable reports. It starts as a small command-line application that counts files in a directory. Over 16 milestones, it becomes a reliable CLI and optional local HTTP service with configuration, parallel scanning, persistence, observability, and releases.

This one project offers natural opportunities to study Rust's ownership model, borrowing, lifetimes, enums, traits, generics, iterators, error handling, filesystem APIs, smart pointers, multithreading, async programming, macros, testing, performance profiling, and release engineering. Advanced or mismatched topics (unsafe Rust, FFI, embedded/no_std, WebAssembly) will be learned through isolated companion labs, not forced into production code.

Project success has two dimensions: (1) software quality and (2) demonstrable understanding. Both must pass before progressing to the next milestone.

#### 1.1 Goals
- Build a real, useful, installable Rust tool rather than a collection of disconnected toy programs.
- Cover core and advanced Rust comprehensively, including their practical trade-offs.
- Practice software engineering: requirements, architecture, tests, CLI UX, security, performance, CI/CD, packaging, and documentation.
- Produce GitHub-ready evidence: explainable code, tests, example reports, benchmarks, design decisions, and release binaries.
- Use Antigravity as a teaching assistant and code reviewer, with user control over implementation and design choices.

#### 1.2 Explicit Non-Goals
- Not a full replacement for Clippy, Semgrep, GitHub code scanning, or language-specific compilers.
- Not an AI code generator or cloud-hosted scanning SaaS in the first release.
- Not a promise to parse every programming language semantically.
- Not an excuse to implement needless microservices, Kubernetes, Kafka, or distributed storage.
- Not a replacement or rewrite of the existing Scala DevForge project.
- Not a guarantee of covering every Rust crate, target platform, unstable feature, or nightly API.

---

### 2. Problem, Users, and Value

#### 2.1 Problem Statement
Developers often need a lightweight, transparent way to understand a repository's composition, detect obvious maintainability signals, and produce repeatable reports. Existing tools can be powerful but specialized. RepoLens will intentionally start with reliable filesystem and text-level analysis, then evolve through measured, test-driven additions.

#### 2.2 Users and Workflows
| Persona | Need | Initial Workflow |
| :--- | :--- | :--- |
| Student / new developer | Learn Rust using realistic inputs | Scan a local practice repository; interpret output |
| Application developer | Understand project footprint | Count files, extensions, lines, and sizes |
| Maintainer | Spot maintainability signals | Find TODO/FIXME markers and oversized files |
| CI operator | Generate reproducible results | Execute CLI and ingest JSON report |
| Advanced local user | Compare scans over time | Store scan history and optionally expose localhost API |

#### 2.3 Product Principles
- **Correct before fast**: baseline serial scanner precedes parallel implementations.
- **Safe by default**: read-only access, no arbitrary execution of scanned repository contents.
- **Progressive complexity**: introduce each dependency or abstraction only when required.
- **Explainable results**: every metric and warning has a documented definition.
- **Deterministic outputs**: stable sorting and reproducible JSON for equivalent repository state/configuration, excluding deliberate timestamps.
- **Offline-first**: local scanning requires no network connection.
- **Cross-platform**: Windows, Linux and macOS, including non-UTF-8 filesystem paths where supported.

---

### 3. Scope and Release Evolution

#### 3.1 Version Plan
| Stage | Proposed Tag | User-Facing Capability | What is intentionally deferred |
| :--- | :--- | :--- | :--- |
| Lesson prototype | v0.1-local | Count immediate files/directories and aggregate sizes | Recursion, configuration, JSON |
| Usable MVP | v0.2 | Recursive scan, file types, size/line counts, summary | Parallelism, DB, API |
| Quality analyzer | v0.3 | Rules, TODO/FIXME, size thresholds, ignore support, JSON | Plugins, distributed scanning |
| Reliable CLI | v0.4 | Formal commands/options, config, robust errors, tests | Async service |
| Performance release | v0.5 | Parallel scan, benchmarks, resource limits | Premature optimization |
| Local service | v0.6 | Optional async HTTP API and persistent scan history | Public internet exposure |
| Portfolio release | v1.0 | Security review, CI, docs, packaging, stable report schema | Enterprise/cloud SaaS |

#### 3.2 MVP Definition (first shippable CLI)
MVP completes a recursive, read-only scan of a single local directory, skips symlinks by default, honors configurable excludes (initially simple patterns, then `.gitignore`-aware rules), counts eligible files/bytes/lines, summarizes by extension, flags TODO/FIXME and oversized files, writes a stable JSON report and displays a readable terminal summary. It emits useful errors rather than crashing for normal invalid inputs.

---

### 4. Functional Requirements
| ID | Requirement | Priority | Earliest Milestone |
| :--- | :--- | :--- | :--- |
| FR-01 | Accept a repository path (default current directory); validate existence and directory type | Must | M1 |
| FR-02 | Count direct children correctly, separating files and directories | Must | M1 |
| FR-03 | Traverse subdirectories recursively without cycles; default not to follow symbolic links | Must | M2–M3 |
| FR-04 | Represent scan results using explicit Rust domain structs and enums | Must | M2 |
| FR-05 | Count eligible files, total bytes, and source lines according to documented rules | Must | M4 |
| FR-06 | Aggregate statistics by normalized file extension | Must | M4 |
| FR-07 | Report inaccessible files and recoverable I/O errors without silently falsifying counts | Must | M5 |
| FR-08 | Support a composable rule engine (TODO/FIXME, file size threshold, later custom rules) | Must | M7 |
| FR-09 | Support CLI options, human text output and versioned JSON output | Must | M6–M11 |
| FR-10 | Honor exclusions and resource guards (max depth, file count, bytes); configure defaults | Must | M5–M11 |
| FR-11 | Provide unit, integration, snapshot/golden, and regression tests | Must | Every milestone |
| FR-12 | Parallelize only where a benchmark shows meaningful benefit | Should | M9 |
| FR-13 | Save and compare local scan history with SQLite | Should | M11 |
| FR-14 | Provide optional localhost-only HTTP endpoints | Could | M13 |
| FR-15 | Support machine-readable CI exit codes for policy violations and runtime errors | Should | M13–M16 |
| FR-16 | Build release artifacts and document installation, usage, limitations and security assumptions | Must | M16 |
| FR-17 | Provide an extensibility example (custom analyzer + macro) without unsafe code | Could | M12 |
| FR-18 | Expose configurable structured logging and basic metrics | Should | M13–M15 |

---

### 5. Non-Functional Requirements and Safety
- Read-only scanning; bounded recursion and memory.
- Handling untrusted paths: no code execution, safe symlink handling, cross-platform paths.
- Metric definitions: files, directories, bytes, lines, TODO markers, oversized files, completeness.

---

### 6. Architecture & Progressive Structure
- Start as single Cargo binary with standard library.
- Evolve to modular architecture: Domain, Scanner, Rules, Application, Presentation, Storage, API.

---

### 7. Implementation Milestones
- **M1**: Rust foundations & one-directory scanner (immediate entries, no recursion, std library only)
- **M2**: Data modeling & recursive traversal
- **M3**: Ownership & borrowing in realistic code
- **M4**: Collections, iterators & file metrics
- **M5**: Error handling, input validation & boundaries
- **M6**: Modules, library API & usable CLI
- **M7**: Traits, generics & rule engine
- **M8**: Smart pointers & interior mutability
- **M9**: Threads & measured parallel scanning
- **M10**: Async Rust & cancellation
- **M11**: Serialization, configuration & scan history
- **M12**: Macros & extension mechanisms
- **M13**: Local HTTP API, tracing & app boundaries
- **M14**: Testing & reliability
- **M15**: Performance engineering
- **M16**: Professional release, documentation & portfolio defense
