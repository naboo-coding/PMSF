# Polymorphic Malware Stage Framework (PMSF)

> **DISCLAIMER:** This framework is a proof‑of‑concept for educational and research use only. The author does not condone or support malicious or unauthorized activities.


PMSF is a research and educational Rust framework for simulating the modular stages of malware, with a focus on polymorphism, configurability, and extensibility. It is safe by default, with all real actions stubbed out, and is designed to help researchers and students understand malware staging and evasion techniques in a controlled, observable way.

---

## Why is PMSF Useful?

- **Safe Malware Simulation:** PMSF allows users to study and simulate malware stages in a controlled, non-destructive environment. All real actions are stubbed out, ensuring safety for research and education.
- **Educational Value:** The framework is designed for students and educators to understand the inner workings of malware staging, evasion, and modularity without risk.
- **Research Utility:** PMSF provides a flexible platform for researchers to experiment with and analyze different malware techniques, configurations, and evasion strategies.
- **Modularity & Extensibility:** Its stage-based, pluggable architecture makes it easy to add, modify, or chain new techniques, supporting a wide range of experiments and demonstrations.
- **Configurable & Observable:** With dynamic configuration and integrated logging/telemetry, users can customize scenarios and observe detailed stage-level events for deeper insight.

---

## Key Features

- **Compile‑time polymorphism** via the `rustmorphism` crate (multiple binaries by build).
- **Dynamic configuration** through a TOML file (`config.toml`) for explicit stage selection.
- **Runtime registration** of custom techniques using a global registry and `register_*` functions.
- **Structured error handling** with the `FrameworkError` enum for clearer diagnostics.
- **Context propagation** with `StageContext`, carrying payloads and metadata between stages.
- **Logging integration** using the `log` crate and `env_logger` for stage‑level events.
- **Telemetry hooks** via the `TelemetryEvent` trait for research or simulation callbacks.
- **Chaining utilities** (`run_*_chain`) to invoke multiple techniques in sequence, collecting errors.
- **Selection helpers**: weighted random (`weighted_random_choice`) and conditional (environment/config) selection.
- **Example/demo** under `examples/demo.rs` showing full usage.

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
    // Default stage context (no payload, empty metadata)
    let ctx = StageContext::default();

    // Compile‑time polymorphic selection
    let p = establish_persistence_poly();
    p.establish_persistence(&ctx).unwrap();

    let a = perform_anti_analysis_poly();
    a.perform_anti_analysis(&ctx).unwrap();

    let e = execute_code_poly();
    e.execute_code(&ctx).unwrap();

    let c = communicate_c2_poly();
    c.communicate_c2(&ctx).unwrap();
}
```

---

## Configuration

Create a `config.toml` in your project root to override defaults or random selection:

```toml
# Choose one technique per stage, or omit to fallback to random.
persistence = "ScheduledTasks"
execution  = "MappingInjection"
c2         = "DNSTunneling"
anti_analysis = "VMDetection"
```

Load it at runtime:

```rust
let config = FrameworkConfig::from_file("config.toml");
println!("Loaded config: {:?}", config);
```

---

## Dynamic Registration

To add your own technique at runtime:

```rust
// Define and implement your stage trait
struct MyTechnique;
impl PersistenceStage for MyTechnique { /* ... */ }

// Register it under a name
register_persistence("MyTechnique", || Box::new(MyTechnique));

// Later lookup by name (or fallback)
if let Some(inst) = get_persistence_by_name("MyTechnique") {
    inst.establish_persistence(&ctx)?;
}
```

---

## Logging & Telemetry

Initialize logging and an optional telemetry callback:

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

All stages will log `info!` and emit `on_event(...)` on success or `error!` on failure.

---

## Chaining Techniques

Run multiple techniques in sequence and gather any errors:

```rust
let errs = run_persistence_chain(&["RegistryRunKeys", "ScheduledTasks"]);
match errs {
    Ok(_) => println!("All persistence steps succeeded."),
    Err(e) => println!("Errors: {:?}", e),
}
```

---

## Helpers

- **Weighted random**: `weighted_random_choice(&[(&"A", 70), (&"B", 30)])`
- **Conditional**: `select_technique_by_condition(&choices, "ENV_VAR")`

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

## Warning

 **SECURITY WARNING:**
 This framework is designed to simulate malware stages for research and education. However, because it is open source, it is possible for someone to modify the code to perform real malicious actions. **Always check the code before running it, especially if you received it from someone else.**

 To verify the code is safe:
 - Open `src/lib.rs`.
 - Search for the following function names:
     - `establish_persistence`
     - `execute_code`
     - `communicate_c2`
     - `perform_anti_analysis`
 - Inside each function, look for comments like `// Placeholder`.
 - If you see only logging and `Ok(())` or `Ok(true)` being returned, it's safe.
  - If you see real system calls, file operations, network connections, or anything that changes your system, that's a red flag.

> **Never run code you do not trust or understand.**

---

## License

The MIT License — see [LICENSE](LICENSE) for details.

