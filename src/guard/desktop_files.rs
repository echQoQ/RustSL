#[cfg(feature = "vm_check_desktop_files")]
#[allow(dead_code)]
pub fn check_desktop_files(threshold: usize) -> bool {
    use std::fs;
    let desktop_path = match get_desktop_path() {
        Some(path) => path,
        None => return false,
    };

    let entries = match fs::read_dir(&desktop_path) {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    let file_count = entries.filter_map(|entry| entry.ok()).count();

    file_count >= threshold
}

#[cfg(feature = "vm_check_desktop_files")]
#[allow(dead_code)]
fn get_desktop_path() -> Option<std::path::PathBuf> {
    dirs::desktop_dir()
}