use crate::{
    def::{
        ImageDosHeader, ImageExportDirectory, ImageNtHeaders, ListEntry, LoaderDataTableEntry,
        PebLoaderData, IMAGE_DIRECTORY_ENTRY_EXCEPTION,
    },
    utils::{dbj2_hash, find_peb, get_cstr_len},
};
use core::ffi::c_ulong;

/// The `syscall!` macro is designed to resolve and invoke system calls.
/// It simplifies the process of finding the system call's address and service number, setting up
/// a hardware breakpoint, and invoking the system call with the specified parameters.
///
/// ### Macro Inputs
/// 1. **`$syscall_name`**: The name of the system call to resolve (typically provided as a string).
/// 2. **`$fn_sig`**: The function signature/type of the system call. This allows the resolved address
///    to be cast to the appropriate function pointer type so it can be invoked directly.
/// 3. **`$($param:expr),*`**: A variadic list of parameters to be passed to the resolved system call
///    function. These parameters are passed as expressions.
#[macro_export]
macro_rules! syscall {
    ($syscall_name:expr, $fn_sig:ty, $($param:expr),*) => {
        {
            let mut syscall_addr: *mut u8 = core::ptr::null_mut();

            // Resolve the system call's address and System Service Number (SSN).
            let ssn = unsafe { $crate::get_ssn_by_name($syscall_name, None, &mut syscall_addr) };

            // Exit if the SSN is invalid or address is `null`.
            if ssn < 0 || syscall_addr.is_null() {
                -1
            } else {
                // Convert the resolved address to a function pointer of the specified type (`$fn_sig`).
                let pt_syscall: $fn_sig = unsafe { core::mem::transmute(syscall_addr) };

                // Set a hardware breakpoint on the system call.
                $crate::set_hw_bp(syscall_addr as usize, 1, ssn as u32);

                // Invoke the system call with the provided parameters (`$param`).
                unsafe { pt_syscall($($param),*) }
            }
        }
    };
}

/// Retrieves the System Service Number (SSN) and the address for a specified syscall.
///
/// This function scans the loaded modules in memory to locate `ntdll.dll`, then utilizes the Exception Directory and
/// Export Address Table to identify the specified syscall. It matches the syscall either by name or an optional hash
/// and returns the SSN if a match is found, along with setting the function’s address in the provided `addr` parameter.
///
/// For more details on this approach, see MDsec's article on using the Exception Directory to resolve System Service Numbers:
/// https://www.mdsec.co.uk/2022/04/resolving-system-service-numbers-using-the-exception-directory/
///
/// # Arguments
/// * `syscall` - The name of the syscall to locate.
/// * `hash` - An optional hash of the syscall name. If provided, the function uses this hash for matching rather than the name.
/// * `addr` - A mutable pointer that will be set to the address of the resolved syscall if a match is found.
///
/// # Returns
/// Returns the SSN (System Service Number) of the matched syscall or -1 if no match is found. If a match is found,
/// `addr` will contain the function's address in `ntdll.dll`.
pub unsafe fn get_ssn_by_name(syscall: &str, hash: Option<usize>, addr: &mut *mut u8) -> i32 {
    let peb = find_peb(); // Get the Process Environment Block (PEB)
    let ldr = (*peb).loader_data as *mut PebLoaderData;

    // Traverse the list of loaded modules in memory.
    let mut next = (*ldr).in_memory_order_module_list.flink;
    let head = &mut (*ldr).in_memory_order_module_list;

    while next != head {
        let ent = (next as *mut u8).offset(-(core::mem::size_of::<ListEntry>() as isize))
            as *mut LoaderDataTableEntry;
        next = (*ent).in_memory_order_links.flink;

        let dll_base = (*ent).dll_base as *const u8;
        let dos_header = dll_base as *const ImageDosHeader;
        let nt_headers = dll_base.offset((*dos_header).e_lfanew as isize) as *const ImageNtHeaders;

        let export_directory_rva = (*nt_headers).optional_header.data_directory[0].virtual_address;
        if export_directory_rva == 0 {
            continue;
        }

        let export_directory =
            dll_base.offset(export_directory_rva as isize) as *const ImageExportDirectory;

        if (*export_directory).number_of_names == 0 {
            continue;
        }

        let dll_name = dll_base.offset((*export_directory).name as isize) as *const u8;
        let dll_name_len = get_cstr_len(dll_name as _);
        let dll_name_str =
            core::str::from_utf8_unchecked(core::slice::from_raw_parts(dll_name, dll_name_len));

        // If module name is not ntdll.dll, skip.
        if dbj2_hash(dll_name_str.as_bytes()) != 0x1edab0ed {
            continue;
        }

        // Retrieve the Exception Directory.
        let rva = (*nt_headers).optional_header.data_directory[IMAGE_DIRECTORY_ENTRY_EXCEPTION]
            .virtual_address;
        if rva == 0 {
            return -1;
        }

        let rtf = (dll_base.offset(rva as isize)) as PimageRuntimeFunctionEntry;

        // Access the Export Address Table.
        let address_of_functions = dll_base
            .offset((*export_directory).address_of_functions as isize)
            as *const core::ffi::c_ulong;
        let address_of_names = dll_base.offset((*export_directory).address_of_names as isize)
            as *const core::ffi::c_ulong;
        let address_of_name_ordinals = dll_base
            .offset((*export_directory).address_of_name_ordinals as isize)
            as *const core::ffi::c_ushort;

        let mut ssn = 0; // Initialize the system call number (SSN).

        // Traverse the runtime function table.
        for i in 0.. {
            let begin_address = (*rtf.offset(i as isize)).begin_address;
            if begin_address == 0 {
                break;
            }

            // Search the export address table.
            for j in 0..(*export_directory).number_of_functions {
                let ordinal = *address_of_name_ordinals.offset(j as isize);
                let function_address = *address_of_functions.offset(ordinal as isize);

                // Check if the function's address matches the runtime function's address.
                if function_address == begin_address {
                    let api_name_addr =
                        dll_base.offset(*address_of_names.offset(j as isize) as isize) as *const u8;
                    let api_name_len = get_cstr_len(api_name_addr as _);
                    let api_name_str = core::str::from_utf8_unchecked(core::slice::from_raw_parts(
                        api_name_addr,
                        api_name_len,
                    ));

                    // Match either by hash or by name.
                    match hash {
                        Some(h) => {
                            if h == dbj2_hash(api_name_str.as_bytes()) as usize {
                                *addr = dll_base.offset(function_address as isize) as *mut u8;
                                return ssn;
                            }
                        }
                        None => {
                            if api_name_str == syscall {
                                *addr = dll_base.offset(function_address as isize) as *mut u8;
                                return ssn;
                            }
                        }
                    }

                    // Increment SSN if the function starts with "Zw" (system call).
                    if api_name_str.starts_with("Zw") {
                        ssn += 1;
                    }
                }
            }
        }
    }

    -1 // Return -1 if no syscall is found.
}

#[repr(C)]
pub struct ImageRuntimeFunctionEntry {
    pub begin_address: c_ulong,
    pub end_address: c_ulong,
    pub u: IMAGE_RUNTIME_FUNCTION_ENTRY_u,
}

#[repr(C)]
pub union IMAGE_RUNTIME_FUNCTION_ENTRY_u {
    pub unwind_info_address: c_ulong,
    pub unwind_data: c_ulong,
}

/// Type alias for pointer to `_IMAGE_RUNTIME_FUNCTION_ENTRY`
pub type PimageRuntimeFunctionEntry = *mut ImageRuntimeFunctionEntry;
