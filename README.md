# Polymorphic Malware Stage Framework (PMSF)

> **A proof-of-concept Rust library for compile-time polymorphic malware stage selection using [rustmorphism](https://crates.io/crates/rustmorphism).**

This library demonstrates how to use compile-time function polymorphism to generate unique malware binaries from a single codebase. Each build deterministically selects a different set of techniques for each malware stage (persistence, execution, C2, anti-analysis), making static signature-based detection much harder.

---

## Table of Contents

- [Features](#features)
- [Motivation](#motivation)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Stage Interfaces](#stage-interfaces)
  - [Polymorphic Implementations](#polymorphic-implementations)
- [How It Works](#how-it-works)
- [Controlling Selection](#controlling-selection)
- [FAQ](#faq)
- [Contributing](#contributing)
- [License](#license)
- [Links](#links)

---

## Features

- **Multiple malware techniques:** Each stage (persistence, execution, C2, anti-analysis) has several implementations.
- **Compile-time polymorphism:** One implementation per stage is selected at build time using `rustmorphism`.
- **Unique binaries per build:** Each build can produce a different combination of techniques, hindering static analysis and signature-based detection.
- **Zero runtime overhead:** Only the selected implementations are included in the final binary.
- **Modular and extensible:** Easily add new techniques for any stage.

---

## Motivation

Traditional malware is often detected by static signatures. By using compile-time polymorphism, this framework demonstrates how a single codebase can generate many unique binaries, each with different embedded techniques. This approach is inspired by real-world polymorphic malware, but is intended for educational and research purposes only.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rustmorphism = "0.1.0"
```

Clone or copy this repository to use the framework as a starting point for your own research.

---

## Quick Start

```rust
use PMSFlib::{
    establish_persistence_poly, perform_anti_analysis_poly, execute_code_poly, communicate_c2_poly
};

fn main() {
    let persistence = establish_persistence_poly();
    persistence.establish_persistence().unwrap();

    let anti_analysis = perform_anti_analysis_poly();
    anti_analysis.perform_anti_analysis().unwrap();

    let execution = execute_code_poly();
    execution.execute_code().unwrap();

    let c2 = communicate_c2_poly();
    c2.communicate_c2().unwrap();
}
```

---

## Usage

### Stage Interfaces

Each malware stage is defined as a Rust trait:

```rust
pub trait PersistenceStage {
    fn establish_persistence(&self) -> Result<(), String>;
}

pub trait ExecutionStage {
    fn execute_code(&self) -> Result<(), String>;
}

pub trait C2Stage {
    fn communicate_c2(&self) -> Result<(), String>;
}

pub trait AntiAnalysisStage {
    fn perform_anti_analysis(&self) -> Result<(), String>;
}
```

### Polymorphic Implementations

For each stage, multiple techniques are implemented and wrapped in a `polymorphic_fn!` macro:

```rust
use rustmorphism::polymorphic_fn;

// Example for persistence
polymorphic_fn! {
    pub fn establish_persistence_poly() -> Box<dyn PersistenceStage> {
        { Box::new(RegistryRunKeys) },
        { Box::new(ScheduledTasks) },
        { Box::new(WMIEventSubscription) }
    }
}
```

The same pattern is used for execution, C2, and anti-analysis stages.

---

## How It Works

The `polymorphic_fn!` macro from `rustmorphism` selects one implementation for each function at compile time, based on build metadata (such as build hash, environment, etc.). This means:

- Each build can select a different set of techniques for each stage.
- Only the selected implementations are included in the binary.
- Analysis of one sample reveals only the chosen set of techniques.

---

## Controlling Selection

You can influence which techniques are selected by:

- Forcing a rebuild (e.g., `cargo clean` or changing a file).
- Modifying the build script to change the build hash or environment variable.
- Using different build environments or metadata.

---

## FAQ

**Q: Is this real malware?**  
A: No. This is a proof-of-concept for research and educational purposes only. All technique implementations are placeholders.

**Q: Can I add my own techniques?**  
A: Yes! Just implement the appropriate trait and add it to the relevant `polymorphic_fn!` macro.

**Q: Is there any runtime overhead?**  
A: No. Only the selected implementation is compiled into the binary.

**Q: How do I know which technique was selected?**  
A: You can print the type name of the selected implementation using `std::any::type_name_of_val`.

---

## Contributing

Contributions, issues, and feature requests are welcome! Feel free to open an issue or submit a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


</rewritten_file>

