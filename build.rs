use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    // Use the current time and a random value for demonstration (replace with real metadata if needed)
    let hash = format!("{}-{}", chrono::Utc::now().timestamp(), rand::random::<u64>());
    println!("cargo:rustc-env=RUSTMORPHISM_BUILD_HASH={}", hash);
} 