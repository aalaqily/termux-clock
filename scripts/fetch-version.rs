#!/usr/bin/env rust-script
// cargo-deps: toml

use std::fs;
use toml;

// Fetch package version and print it
// IMPORTANT
// Only use in package root directory

fn main() {
    // Read the Cargo.toml file
    let cargo_toml = fs::read_to_string("Cargo.toml")
        .expect("Failed to read Cargo.toml");

    // Parse the content using the toml crate
    let parsed: toml::Value = toml::de::from_str(&cargo_toml).expect("Failed to parse Cargo.toml");

    // Extract the version
    if let Some(version) = parsed.get("package").and_then(|p| p.get("version")) {
        println!("{}", version.as_str().unwrap());
    } else {
        println!("Version not found in Cargo.toml");
    }
}
