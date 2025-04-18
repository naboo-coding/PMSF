use rustmorphism::polymorphic_fn;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use log::{info, error};
use rand::prelude::*;
use std::env;
use std::fmt;
use std::fs;
use toml;

/// Context passed to each stage, carrying optional payload and shared metadata.
#[derive(Debug)]
pub struct StageContext {
    /// Optional binary payload for stages that need input data.
    pub payload: Option<Vec<u8>>,
    /// Shared key-value metadata for stage operations.
    pub metadata: HashMap<String, String>,
}

/// Custom error type for framework stage operations, providing more descriptive errors.
#[derive(Debug)]
pub enum FrameworkError {
    /// A generic error with a message.
    Generic(String),
}

// Implement Display for FrameworkError to enable proper logging of errors
impl fmt::Display for FrameworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameworkError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

/// Trait defining persistence techniques.
/// Implementors may use `StageContext` for input and return `FrameworkError` on error.
pub trait PersistenceStage {
    /// Establish persistence given the provided context.
    fn establish_persistence(&self, ctx: &StageContext) -> Result<(), FrameworkError>;
}

/// Trait defining code execution techniques.
/// Implementors may use `StageContext` and return a payload or `FrameworkError` on error.
pub trait ExecutionStage {
    /// Execute code given the provided context and produce any outbound payload.
    fn execute_code(&self, ctx: &StageContext) -> Result<Vec<u8>, FrameworkError>;
}

/// Trait defining C2 communication techniques.
/// Implementors may use `StageContext` and return `FrameworkError` on error.
pub trait C2Stage {
    /// Perform C2 communication given the provided context.
    fn communicate_c2(&self, ctx: &StageContext) -> Result<(), FrameworkError>;
}

/// Trait defining anti-analysis techniques.
/// Implementors may use `StageContext` and return `Ok(true)` if evasion succeeded or `FrameworkError` on error.
pub trait AntiAnalysisStage {
    /// Perform anti-analysis given the provided context.
    fn perform_anti_analysis(&self, ctx: &StageContext) -> Result<bool, FrameworkError>;
}

// Registries for dynamic techniques
pub static PERSISTENCE_REGISTRY: Lazy<Mutex<HashMap<String, fn() -> Box<dyn PersistenceStage>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static EXECUTION_REGISTRY: Lazy<Mutex<HashMap<String, fn() -> Box<dyn ExecutionStage>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static C2_REGISTRY: Lazy<Mutex<HashMap<String, fn() -> Box<dyn C2Stage>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static ANTI_ANALYSIS_REGISTRY: Lazy<Mutex<HashMap<String, fn() -> Box<dyn AntiAnalysisStage>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

// Registration functions
pub fn register_persistence(name: &str, constructor: fn() -> Box<dyn PersistenceStage>) {
    PERSISTENCE_REGISTRY.lock().unwrap().insert(name.to_string(), constructor);
}
pub fn register_execution(name: &str, constructor: fn() -> Box<dyn ExecutionStage>) {
    EXECUTION_REGISTRY.lock().unwrap().insert(name.to_string(), constructor);
}
pub fn register_c2(name: &str, constructor: fn() -> Box<dyn C2Stage>) {
    C2_REGISTRY.lock().unwrap().insert(name.to_string(), constructor);
}
pub fn register_anti_analysis(name: &str, constructor: fn() -> Box<dyn AntiAnalysisStage>) {
    ANTI_ANALYSIS_REGISTRY.lock().unwrap().insert(name.to_string(), constructor);
}

// Persistence Implementations
pub struct RegistryRunKeys;
impl PersistenceStage for RegistryRunKeys {
    fn establish_persistence(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[Persistence] RegistryRunKeys: Attempting to establish persistence");
        // Placeholder for Registry Run Keys logic
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[Persistence] RegistryRunKeys: Success");
                emit_telemetry("Persistence", "RegistryRunKeys", "Success");
            },
            Err(e) => error!("[Persistence] RegistryRunKeys: Error: {}", e),
        }
        result
    }
}

pub struct ScheduledTasks;
impl PersistenceStage for ScheduledTasks {
    fn establish_persistence(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[Persistence] ScheduledTasks: Attempting to establish persistence");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[Persistence] ScheduledTasks: Success");
                emit_telemetry("Persistence", "ScheduledTasks", "Success");
            },
            Err(e) => error!("[Persistence] ScheduledTasks: Error: {}", e),
        }
        result
    }
}

pub struct WMIEventSubscription;
impl PersistenceStage for WMIEventSubscription {
    fn establish_persistence(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[Persistence] WMIEventSubscription: Attempting to establish persistence");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[Persistence] WMIEventSubscription: Success");
                emit_telemetry("Persistence", "WMIEventSubscription", "Success");
            },
            Err(e) => error!("[Persistence] WMIEventSubscription: Error: {}", e),
        }
        result
    }
}

polymorphic_fn! {
    pub fn establish_persistence_poly() -> Box<dyn PersistenceStage> {
        { Box::new(RegistryRunKeys) },
        { Box::new(ScheduledTasks) },
        { Box::new(WMIEventSubscription) }
    }
}

// Execution Implementations
pub struct ClassicProcessInjection;
impl ExecutionStage for ClassicProcessInjection {
    fn execute_code(&self, ctx: &StageContext) -> Result<Vec<u8>, FrameworkError> {
        info!("[Execution] ClassicProcessInjection: Attempting to execute code");
        let result = Ok(Vec::new());
        match &result {
            Ok(_) => {
                info!("[Execution] ClassicProcessInjection: Success");
                emit_telemetry("Execution", "ClassicProcessInjection", "Success");
            },
            Err(e) => error!("[Execution] ClassicProcessInjection: Error: {}", e),
        }
        result
    }
}

pub struct MappingInjection;
impl ExecutionStage for MappingInjection {
    fn execute_code(&self, ctx: &StageContext) -> Result<Vec<u8>, FrameworkError> {
        info!("[Execution] MappingInjection: Attempting to execute code");
        let result = Ok(Vec::new());
        match &result {
            Ok(_) => {
                info!("[Execution] MappingInjection: Success");
                emit_telemetry("Execution", "MappingInjection", "Success");
            },
            Err(e) => error!("[Execution] MappingInjection: Error: {}", e),
        }
        result
    }
}

pub struct ThreadHijacking;
impl ExecutionStage for ThreadHijacking {
    fn execute_code(&self, ctx: &StageContext) -> Result<Vec<u8>, FrameworkError> {
        info!("[Execution] ThreadHijacking: Attempting to execute code");
        let result = Ok(Vec::new());
        match &result {
            Ok(_) => {
                info!("[Execution] ThreadHijacking: Success");
                emit_telemetry("Execution", "ThreadHijacking", "Success");
            },
            Err(e) => error!("[Execution] ThreadHijacking: Error: {}", e),
        }
        result
    }
}

pub struct DirectSyscallExecution;
impl ExecutionStage for DirectSyscallExecution {
    fn execute_code(&self, ctx: &StageContext) -> Result<Vec<u8>, FrameworkError> {
        info!("[Execution] DirectSyscallExecution: Attempting to execute code");
        let result = Ok(Vec::new());
        match &result {
            Ok(_) => {
                info!("[Execution] DirectSyscallExecution: Success");
                emit_telemetry("Execution", "DirectSyscallExecution", "Success");
            },
            Err(e) => error!("[Execution] DirectSyscallExecution: Error: {}", e),
        }
        result
    }
}

polymorphic_fn! {
    pub fn execute_code_poly() -> Box<dyn ExecutionStage> {
        { Box::new(ClassicProcessInjection) },
        { Box::new(MappingInjection) },
        { Box::new(ThreadHijacking) },
        { Box::new(DirectSyscallExecution) }
    }
}

// C2 Communication Implementations
pub struct HTTPSCommunication;
impl C2Stage for HTTPSCommunication {
    fn communicate_c2(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[C2] HTTPSCommunication: Attempting C2 communication");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[C2] HTTPSCommunication: Success");
                emit_telemetry("C2", "HTTPSCommunication", "Success");
            },
            Err(e) => error!("[C2] HTTPSCommunication: Error: {}", e),
        }
        result
    }
}

pub struct DNSTunneling;
impl C2Stage for DNSTunneling {
    fn communicate_c2(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[C2] DNSTunneling: Attempting C2 communication");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[C2] DNSTunneling: Success");
                emit_telemetry("C2", "DNSTunneling", "Success");
            },
            Err(e) => error!("[C2] DNSTunneling: Error: {}", e),
        }
        result
    }
}

pub struct ICMPCommunication;
impl C2Stage for ICMPCommunication {
    fn communicate_c2(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[C2] ICMPCommunication: Attempting C2 communication");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[C2] ICMPCommunication: Success");
                emit_telemetry("C2", "ICMPCommunication", "Success");
            },
            Err(e) => error!("[C2] ICMPCommunication: Error: {}", e),
        }
        result
    }
}

pub struct RawSocketsCommunication;
impl C2Stage for RawSocketsCommunication {
    fn communicate_c2(&self, ctx: &StageContext) -> Result<(), FrameworkError> {
        info!("[C2] RawSocketsCommunication: Attempting C2 communication");
        let result = Ok(());
        match &result {
            Ok(_) => {
                info!("[C2] RawSocketsCommunication: Success");
                emit_telemetry("C2", "RawSocketsCommunication", "Success");
            },
            Err(e) => error!("[C2] RawSocketsCommunication: Error: {}", e),
        }
        result
    }
}

polymorphic_fn! {
    pub fn communicate_c2_poly() -> Box<dyn C2Stage> {
        { Box::new(HTTPSCommunication) },
        { Box::new(DNSTunneling) },
        { Box::new(ICMPCommunication) },
        { Box::new(RawSocketsCommunication) }
    }
}

// Anti-Analysis Implementations
pub struct AntiDebugging;
impl AntiAnalysisStage for AntiDebugging {
    fn perform_anti_analysis(&self, ctx: &StageContext) -> Result<bool, FrameworkError> {
        info!("[AntiAnalysis] AntiDebugging: Performing anti-analysis");
        let result = Ok(true);
        match &result {
            Ok(_) => {
                info!("[AntiAnalysis] AntiDebugging: Success");
                emit_telemetry("AntiAnalysis", "AntiDebugging", "Success");
            },
            Err(e) => error!("[AntiAnalysis] AntiDebugging: Error: {}", e),
        }
        result
    }
}

pub struct VMDetection;
impl AntiAnalysisStage for VMDetection {
    fn perform_anti_analysis(&self, ctx: &StageContext) -> Result<bool, FrameworkError> {
        info!("[AntiAnalysis] VMDetection: Performing anti-analysis");
        let result = Ok(true);
        match &result {
            Ok(_) => {
                info!("[AntiAnalysis] VMDetection: Success");
                emit_telemetry("AntiAnalysis", "VMDetection", "Success");
            },
            Err(e) => error!("[AntiAnalysis] VMDetection: Error: {}", e),
        }
        result
    }
}

pub struct SandboxEvasion;
impl AntiAnalysisStage for SandboxEvasion {
    fn perform_anti_analysis(&self, ctx: &StageContext) -> Result<bool, FrameworkError> {
        info!("[AntiAnalysis] SandboxEvasion: Performing anti-analysis");
        let result = Ok(true);
        match &result {
            Ok(_) => {
                info!("[AntiAnalysis] SandboxEvasion: Success");
                emit_telemetry("AntiAnalysis", "SandboxEvasion", "Success");
            },
            Err(e) => error!("[AntiAnalysis] SandboxEvasion: Error: {}", e),
        }
        result
    }
}

polymorphic_fn! {
    pub fn perform_anti_analysis_poly() -> Box<dyn AntiAnalysisStage> {
        { Box::new(AntiDebugging) },
        { Box::new(VMDetection) },
        { Box::new(SandboxEvasion) }
    }
}

#[derive(Debug, Deserialize)]
pub struct FrameworkConfig {
    pub persistence: Option<String>,
    pub execution: Option<String>,
    pub c2: Option<String>,
    pub anti_analysis: Option<String>,
}

// Implement a helper to load configuration from a TOML file
impl FrameworkConfig {
    /// Load configuration from the given TOML file path, returning `None` on error.
    pub fn from_file(path: &str) -> Option<Self> {
        fs::read_to_string(path)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
    }
}

// Lookup functions for dynamic techniques
pub fn get_persistence_by_name(name: &str) -> Option<Box<dyn PersistenceStage>> {
    PERSISTENCE_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
pub fn get_execution_by_name(name: &str) -> Option<Box<dyn ExecutionStage>> {
    EXECUTION_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
pub fn get_c2_by_name(name: &str) -> Option<Box<dyn C2Stage>> {
    C2_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}
pub fn get_anti_analysis_by_name(name: &str) -> Option<Box<dyn AntiAnalysisStage>> {
    ANTI_ANALYSIS_REGISTRY.lock().unwrap().get(name).map(|ctor| ctor())
}

// --- Telemetry Hooks ---
//
pub trait TelemetryEvent: Send + Sync {
    fn on_event(&self, stage: &str, technique: &str, status: &str);
}

static TELEMETRY_CALLBACK: once_cell::sync::Lazy<Mutex<Option<Box<dyn TelemetryEvent>>>> = once_cell::sync::Lazy::new(|| Mutex::new(None));

pub fn set_telemetry_callback(cb: Box<dyn TelemetryEvent>) {
    *TELEMETRY_CALLBACK.lock().unwrap() = Some(cb);
}

fn emit_telemetry(stage: &str, technique: &str, status: &str) {
    if let Some(cb) = &*TELEMETRY_CALLBACK.lock().unwrap() {
        cb.on_event(stage, technique, status);
    }
}

// --- Dynamic Technique Registration and Lookup ---
//
// To add a new technique (e.g., from a plugin or at runtime):
// 1. Implement the appropriate trait for your technique (e.g., PersistenceStage).
// 2. Register your technique by calling the corresponding register_* function, e.g.:
//      register_persistence("MyTechnique", || Box::new(MyTechnique));
// 3. To use a technique by name, call the corresponding get_*_by_name function, e.g.:
//      if let Some(tech) = get_persistence_by_name("MyTechnique") { ... }
//
// This allows dynamic extension of the framework without recompiling the core.
//
// Example (in a plugin or main):
//      struct MyTechnique;
//      impl PersistenceStage for MyTechnique { ... }
//      register_persistence("MyTechnique", || Box::new(MyTechnique));
//      let instance = get_persistence_by_name("MyTechnique");

/// Weighted random selection helper
pub fn weighted_random_choice<'a, T>(choices: &'a [(&T, u32)]) -> Option<&'a T> {
    let total_weight: u32 = choices.iter().map(|(_, w)| *w).sum();
    if total_weight == 0 { return None; }
    let mut rng = rand::thread_rng();
    let mut roll = rng.gen_range(0..total_weight);
    for (item, weight) in choices {
        if roll < *weight {
            return Some(item);
        }
        roll -= *weight;
    }
    None
}
// Example usage:
// let techniques = [("RegistryRunKeys", 70), ("ScheduledTasks", 20), ("WMIEventSubscription", 10)];
// let selected = weighted_random_choice(&techniques); 

/// Example: Conditional selection based on config or environment
fn select_technique_by_condition<'a, T>(choices: &'a [T], env_var: &str) -> Option<&'a T>
where T: std::fmt::Debug
{
    if let Ok(val) = env::var(env_var) {
        for item in choices {
            if format!("{:?}", item).contains(&val) {
                return Some(item);
            }
        }
    }
    choices.get(0)
}
// Example usage:
// let techniques = ["RegistryRunKeys", "ScheduledTasks", "WMIEventSubscription"];
// let selected = select_technique_by_condition(&techniques, "PERSISTENCE_TECHNIQUE"); 

// --- Chaining multiple techniques per stage ---
/// Run multiple persistence techniques in order, collecting any errors.
pub fn run_persistence_chain(names: &[&str]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for &name in names {
        let stage = get_persistence_by_name(name).unwrap_or_else(establish_persistence_poly);
        if let Err(e) = stage.establish_persistence(&StageContext { payload: None, metadata: HashMap::new() }) {
            errors.push(format!("Persistence {} failed: {}", name, e));
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Run multiple execution techniques in order, collecting any errors.
pub fn run_execution_chain(names: &[&str]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for &name in names {
        let exec = get_execution_by_name(name).unwrap_or_else(execute_code_poly);
        if let Err(e) = exec.execute_code(&StageContext { payload: None, metadata: HashMap::new() }) {
            errors.push(format!("Execution {} failed: {}", name, e));
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Run multiple C2 techniques in order, collecting any errors.
pub fn run_c2_chain(names: &[&str]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for &name in names {
        let c2 = get_c2_by_name(name).unwrap_or_else(communicate_c2_poly);
        if let Err(e) = c2.communicate_c2(&StageContext { payload: None, metadata: HashMap::new() }) {
            errors.push(format!("C2 {} failed: {}", name, e));
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}

/// Run multiple anti-analysis techniques in order, collecting any errors.
pub fn run_anti_analysis_chain(names: &[&str]) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    for &name in names {
        let anti = get_anti_analysis_by_name(name).unwrap_or_else(perform_anti_analysis_poly);
        if let Err(e) = anti.perform_anti_analysis(&StageContext { payload: None, metadata: HashMap::new() }) {
            errors.push(format!("AntiAnalysis {} failed: {}", name, e));
        }
    }
    if errors.is_empty() { Ok(()) } else { Err(errors) }
}