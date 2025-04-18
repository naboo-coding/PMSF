//! # PMSF Framework Demo
//!
//! Demonstrates how to initialize the PMSF framework, load optional configuration,
//! set up telemetry, and run chains of techniques for each stage.
//!
//! **Usage:**
//! ```
//! cargo run --example demo
//! ```

use env_logger;
use log::LevelFilter;
use pmsf::{
    FrameworkConfig,
    StageContext,
    set_telemetry_callback,
    TelemetryEvent,
    run_persistence_chain,
    run_execution_chain,
    run_c2_chain,
    run_anti_analysis_chain,
};

/// Simple telemetry implementation that prints events to the console.
struct ConsoleTelemetry;

impl TelemetryEvent for ConsoleTelemetry {
    fn on_event(&self, stage: &str, technique: &str, status: &str) {
        println!("[Telemetry] {}::{}, status={}", stage, technique, status);
    }
}

fn main() {
    // 1. Set up logging at INFO level
    env_logger::builder().filter_level(LevelFilter::Info).init();

    // 2. Register a telemetry callback for event monitoring
    set_telemetry_callback(Box::new(ConsoleTelemetry));

    // 3. Load configuration file if present (e.g., `config.toml` in project root)
    let config = FrameworkConfig::from_file("config.toml");
    println!("Loaded configuration: {:?}", config);

    // 4. Prepare a default context for stages (no payload, empty metadata)
    let ctx = StageContext { payload: None, metadata: Default::default() };

    // 5. Demonstrate persistence stage chain
    println!("--> Running persistence chain...");
    let _ = run_persistence_chain(&["RegistryRunKeys", "ScheduledTasks"]);

    // 6. Demonstrate execution stage chain
    println!("--> Running execution chain...");
    let _ = run_execution_chain(&["ClassicProcessInjection", "MappingInjection"]);

    // 7. Demonstrate C2 stage chain
    println!("--> Running C2 chain...");
    let _ = run_c2_chain(&["HTTPSCommunication", "DNSTunneling"]);

    // 8. Demonstrate anti-analysis stage chain
    println!("--> Running anti-analysis chain...");
    let _ = run_anti_analysis_chain(&["AntiDebugging", "VMDetection"]);

    println!("Demo completed successfully.");
} 