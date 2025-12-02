const _VC_LTL_VERSION: &'static str = "5.2.2";
const _YY_THUNKS_VERSION: &'static str = "1.1.7";

use std::{env, path::PathBuf};

pub fn thunk() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();

    if target_os != "windows" || target_env != "msvc" {
        println!("cargo:note=Skipped! Only Windows(MSVC) is supported!");
        return;
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let vc_ltl_arch = if target_arch == "x86" { "Win32" } else { "x64" };
    let vc_ltl_platform = "6.0.6000.0";  // Hardcoded for win7

    let vc_ltl_path = if let Ok(vc_ltl_env) = env::var("VC_LTL") {
        PathBuf::from(vc_ltl_env).join(&format!(
            "TargetPlatform/{}/lib/{}",
            vc_ltl_platform, vc_ltl_arch
        ))
    } else {
        println!("cargo:note=VC_LTL environment variable not set, skipping VC-LTL5");
        return;
    };

    println!("cargo::rustc-link-search={}", vc_ltl_path.to_string_lossy());
    println!(
        "cargo:note=VC-LTL5 Enabled: {}({})",
        vc_ltl_platform, vc_ltl_arch
    );

    let yy_thunks_arch = if target_arch == "x86" { "x86" } else { "x64" };
    let yy_thunks_platform = "Win7";  // Hardcoded for win7

    let yy_thunks = if let Ok(yy_thunks_env) = env::var("YY_THUNKS") {
        PathBuf::from(yy_thunks_env).join(format!(
            "objs/{}/YY_Thunks_for_{}.obj",
            yy_thunks_arch, yy_thunks_platform
        ))
    } else {
        println!("cargo:note=YY_THUNKS environment variable not set, skipping YY-Thunks");
        return;
    };

    println!("cargo::rustc-link-arg={}", yy_thunks.to_string_lossy());
    println!(
        "cargo:note=YY-Thunks Enabled: {}({})",
        yy_thunks_platform, yy_thunks_arch
    );

    if true && env::var("PROFILE").unwrap() != "debug" {  // subsystem_windows hardcoded
        println!("cargo::rustc-link-arg=/SUBSYSTEM:WINDOWS");
        println!("cargo::rustc-link-arg=/ENTRY:mainCRTStartup");
        println!("cargo:note=Subsystem is set to WINDOWS");
    } else {
        println!("cargo::rustc-link-arg=/SUBSYSTEM:CONSOLE");
    }
}