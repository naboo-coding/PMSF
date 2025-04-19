# Polymorphic Malware Stage Framework (PMSF)

<p align="left">
  <a href="https://crates.io/crates/pmsf"><img src="https://img.shields.io/crates/v/pmsf.svg?style=flat-square" alt="Crates.io"></a>
  <a href="https://docs.rs/pmsf"><img src="https://img.shields.io/docsrs/pmsf?style=flat-square" alt="docs.rs"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square" alt="License"></a>
</p>

> **⚠️ DISCLAIMER:**
> This framework is a proof‑of‑concept for educational and research use only. The author does not condone or support malicious or unauthorized activities.

PMSF is a robust, research-grade Rust framework engineered for the simulation and analysis of modular malware stages. Designed with a focus on safety, extensibility, and clarity, PMSF empowers security researchers, educators, and students to explore advanced malware staging, evasion, and polymorphism techniques in a controlled, observable environment. All real-world actions are stubbed by default, ensuring a safe and risk-free experience for experimentation and learning.

---

## ⚠️ Security Warning

> **This framework is designed solely for safe simulation of malware stages in research and educational contexts.**
>
> Because PMSF is open source, it is possible for someone to modify the code to perform real malicious actions. **Always review the code before running it, especially if you received it from an untrusted source.**
>
> To verify the code is safe:
> - Open `src/lib.rs`.
> - Search for the following function names:
>     - `establish_persistence`
>     - `execute_code`
>     - `communicate_c2`
>     - `perform_anti_analysis`
> - Inside each function, look for comments like `// Placeholder`.
> - If you see only logging and `Ok(())` or `Ok(true)` being returned, it's safe.
> - If you see real system calls, file operations, network connections, or anything that changes your system, that's a red flag.
>
> **Never run code you do not trust or understand.**

---

## Table of Contents

- [Why Choose PMSF?](#why-choose-pmsf)
- [Key Features](#key-features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Dynamic Registration](#dynamic-registration)
- [Logging & Telemetry](#logging--telemetry)
- [Chaining Techniques](#chaining-techniques)
- [Helpers](#helpers)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)

---

## Why Choose PMSF?

- **Purpose-Built for Security Research & Education:** PMSF is tailored for security professionals, educators, and students seeking a safe, hands-on environment to study and demonstrate malware staging and evasion techniques.
- **Risk-Free Simulation:** All operational stages are fully stubbed, guaranteeing a non-destructive, controlled experience ideal for experimentation and curriculum development.
- **Modular & Extensible Architecture:** Easily add, modify, or chain new techniques with a stage-based, pluggable design—enabling rapid prototyping and comparative analysis.
- **Comprehensive Observability:** Integrated logging and telemetry provide deep visibility into stage-level events, supporting detailed analysis and reporting.
- **Configurable & Reproducible:** Dynamic configuration and deterministic selection mechanisms allow for precise scenario customization and repeatable experiments.

---

## Key Features

- **Compile-Time Polymorphism:** Leverage the `rustmorphism` crate to generate multiple binaries at build time, enabling diverse technique selection.
- **Dynamic Configuration:** Select and orchestrate stages explicitly via a TOML configuration file (`config.toml`).
- **Runtime Extensibility:** Register custom techniques on the fly using a global registry and intuitive `register_*` functions.
- **Structured Error Handling:** Diagnose issues efficiently with the comprehensive `FrameworkError` enum.
- **Context Propagation:** Seamlessly transfer payloads and metadata between stages using the `StageContext` abstraction.
- **Integrated Logging:** Monitor stage-level events in real time with the `log` crate and `env_logger` integration.
- **Telemetry Hooks:** Instrument research and simulation workflows with the `TelemetryEvent` trait for custom callbacks.
- **Chaining Utilities:** Execute multiple techniques in sequence with `run_*_chain`, aggregating results and errors.
- **Advanced Selection Helpers:** Employ weighted random (`weighted_random_choice`) and conditional (environment/config) selection utilities for flexible technique orchestration.
- **Comprehensive Example:** Explore a full-featured demonstration in `examples/demo.rs` showcasing all major capabilities.

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pmsf = "0.1.0"
```

Then import in Rust:

```rust
use pmsf::*;
```

---

## Quick Start

```rust
use pmsf::{
    establish_persistence_poly,
    execute_code_poly,
    communicate_c2_poly,
    perform_anti_analysis_poly,
    StageContext,
};

fn main() {
    // Initialize a default stage context (no payload, empty metadata)
    let ctx = StageContext::default();

    // Select and execute the persistence stage
    let persistence = establish_persistence_poly();
    persistence.establish_persistence(&ctx).expect("Persistence stage failed");

    // Select and execute the anti-analysis stage
    let anti_analysis = perform_anti_analysis_poly();
    anti_analysis.perform_anti_analysis(&ctx).expect("Anti-analysis stage failed");

    // Select and execute the code execution stage
    let execution = execute_code_poly();
    execution.execute_code(&ctx).expect("Execution stage failed");

    // Select and execute the C2 communication stage
    let c2 = communicate_c2_poly();
    c2.communicate_c2(&ctx).expect("C2 stage failed");
}
```

---

## Configuration

To customize stage selection, create a `config.toml` file in your project root. This allows you to explicitly specify which technique to use for each stage, or omit entries to enable random selection.

```toml
# config.toml
# Specify one technique per stage, or omit to use random selection.
persistence = "ScheduledTasks"
execution  = "MappingInjection"
c2         = "DNSTunneling"
anti_analysis = "VMDetection"
```

Load the configuration at runtime:

```rust
let config = FrameworkConfig::from_file("config.toml");
println!("Loaded config: {:?}", config);
```

---

## Dynamic Registration

You can register your own custom technique at runtime, enabling rapid prototyping and extension:

```rust
// Define and implement your stage trait
struct MyTechnique;
impl PersistenceStage for MyTechnique { /* ... */ }

// Register it under a unique name
register_persistence("MyTechnique", || Box::new(MyTechnique));

// Retrieve and use the registered technique by name
if let Some(inst) = get_persistence_by_name("MyTechnique") {
    inst.establish_persistence(&ctx)?;
}
```

---

## Logging & Telemetry

PMSF supports integrated logging and custom telemetry callbacks for enhanced observability:

```rust
use log::LevelFilter;
use env_logger;

env_logger::builder().filter_level(LevelFilter::Info).init();

struct ConsoleTelemetry;
impl TelemetryEvent for ConsoleTelemetry {
    fn on_event(&self, stage: &str, tech: &str, status: &str) {
        println!("Telemetry: {}::{}, {}", stage, tech, status);
    }
}

set_telemetry_callback(Box::new(ConsoleTelemetry));
```

All stages emit `info!` logs and trigger `on_event(...)` on success, or `error!` on failure.

---

## Chaining Techniques

You can execute multiple techniques in sequence and collect any errors for robust, multi-step simulations:

```rust
let errs = run_persistence_chain(&["RegistryRunKeys", "ScheduledTasks"]);
match errs {
    Ok(_) => println!("All persistence steps succeeded."),
    Err(e) => println!("Errors: {:?}", e),
}
```

---

## Helpers

PMSF provides utility functions for advanced technique selection:

- **Weighted random selection:**
  ```rust
  weighted_random_choice(&[("A", 70), ("B", 30)])
  ```
- **Conditional selection based on environment/config:**
  ```rust
  select_technique_by_condition(&choices, "ENV_VAR")
  ```

---

## Examples

Build and run the comprehensive demo:

```bash
cargo run --example demo
```

This example demonstrates config loading, registration, logging, telemetry, and chaining.

---

## Contributing

Issues, pull requests, and suggestions are welcome. Please follow standard Rust project conventions.

---

## License

The MIT License — see [LICENSE](LICENSE) for details.

