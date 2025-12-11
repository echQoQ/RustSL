use core::arch::asm;
use core::ptr::null_mut;

use crate::def::{
    ImageDosHeader, ImageNtHeaders, LoaderDataTableEntry, PebLoaderData, IMAGE_DOS_SIGNATURE,
    IMAGE_NT_SIGNATURE, PEB,
};

/// Retrieves the NT headers from the base address of a module.
///
/// # Arguments
/// * `base_addr` - The base address of the module.
///
/// Returns a pointer to `ImageNtHeaders` or null if the headers are invalid.
#[cfg(target_arch = "x86_64")]
pub unsafe fn get_nt_headers(base_addr: *mut u8) -> *mut ImageNtHeaders {
    let dos_header = base_addr as *mut ImageDosHeader;

    // Check if the DOS signature is valid (MZ)
    if (*dos_header).e_magic != IMAGE_DOS_SIGNATURE {
        return null_mut();
    }

    // Calculate the address of NT headers
    let nt_headers = (base_addr as isize + (*dos_header).e_lfanew as isize) as *mut ImageNtHeaders;

    // Check if the NT signature is valid (PE\0\0)
    if (*nt_headers).signature != IMAGE_NT_SIGNATURE as _ {
        return null_mut();
    }

    nt_headers
}

/// Finds and returns the base address and size of a module by its hash.
///
/// # Arguments
/// * `module_hash` - The hash of the module name to locate.
///
/// Returns a tuple with the base address and the size of the module or (null, 0) if not found.
pub unsafe fn ldr_module_info(module_hash: u32) -> (*mut u8, usize) {
    let peb = find_peb(); // Retrieve the PEB

    if peb.is_null() {
        return (null_mut(), 0);
    }

    let peb_ldr_data_ptr = (*peb).loader_data as *mut PebLoaderData;
    if peb_ldr_data_ptr.is_null() {
        return (null_mut(), 0);
    }

    // Start with the first module in the InLoadOrderModuleList
    let mut module_list =
        (*peb_ldr_data_ptr).in_load_order_module_list.flink as *mut LoaderDataTableEntry;

    // Iterate through the list of loaded modules
    while !(*module_list).dll_base.is_null() {
        let dll_buffer_ptr = (*module_list).base_dll_name.buffer;
        let dll_length = (*module_list).base_dll_name.length as usize;

        // Create a slice from the DLL name
        let dll_name_slice = core::slice::from_raw_parts(dll_buffer_ptr as *const u8, dll_length);

        // Compare the hash of the DLL name with the provided hash
        if module_hash == dbj2_hash(dll_name_slice) {
            let dll_base = (*module_list).dll_base as *const ImageDosHeader;
            let nt_headers = (dll_base as *const u8).offset((*dll_base).e_lfanew as isize)
                as *const ImageNtHeaders;

            // Obtain the size of the module from the OptionalHeader's SizeOfImage
            let size_of_image = (*nt_headers).optional_header.size_of_image as usize;

            return ((*module_list).dll_base as _, size_of_image); // Return the base address and size of the module
        }

        // Move to the next module in the list
        module_list = (*module_list).in_load_order_links.flink as *mut LoaderDataTableEntry;
    }

    (null_mut(), 0)
}

/// Computes the DJB2 hash for the given buffer
pub fn dbj2_hash(buffer: &[u8]) -> u32 {
    let mut hsh: u32 = 5381;
    let mut iter: usize = 0;
    let mut cur: u8;

    while iter < buffer.len() {
        cur = buffer[iter];

        if cur == 0 {
            iter += 1;
            continue;
        }

        if cur >= ('a' as u8) {
            cur -= 0x20;
        }

        hsh = ((hsh << 5).wrapping_add(hsh)) + cur as u32;
        iter += 1;
    }
    hsh
}

/// Calculates the length of a C-style null-terminated string.
pub fn get_cstr_len(pointer: *const char) -> usize {
    let mut tmp: u64 = pointer as u64;

    unsafe {
        while *(tmp as *const u8) != 0 {
            tmp += 1;
        }
    }

    (tmp - pointer as u64) as _
}

/// Finds and returns the Process Environment Block (PEB)
#[cfg(target_arch = "x86_64")]
pub fn find_peb() -> *mut PEB {
    let peb_ptr: *mut PEB;
    unsafe {
        asm!(
        "mov {}, gs:[0x60]",
        out(reg) peb_ptr
        );
    }
    peb_ptr
}
