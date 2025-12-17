#![allow(non_snake_case, non_camel_case_types)]

use std::ffi::c_void;

#[repr(C)]
struct UNICODE_STRING {
    Length: u16,
    MaximumLength: u16,
    Buffer: *mut u16,
}

#[repr(C)]
struct LIST_ENTRY {
    Flink: *mut LIST_ENTRY,
    Blink: *mut LIST_ENTRY,
}

#[repr(C)]
struct PEB_LDR_DATA {
    Length: u32,
    Initialized: u8,
    _padding: [u8; 3],
    SsHandle: *mut c_void,
    InLoadOrderModuleList: LIST_ENTRY,
    InMemoryOrderModuleList: LIST_ENTRY,
    InInitializationOrderLinks: LIST_ENTRY,
}

#[repr(C)]
struct PEB {
    Reserved1: [u8; 2],
    BeingDebugged: u8,
    Reserved2: [u8; 1],
    Reserved3: [*mut c_void; 2],
    Ldr: *mut PEB_LDR_DATA,
}

#[repr(C)]
struct LDR_DATA_TABLE_ENTRY {
    InLoadOrderLinks: LIST_ENTRY,
    InMemoryOrderLinks: LIST_ENTRY,
    InInitializationOrderLinks: LIST_ENTRY,
    DllBase: *mut c_void,
    EntryPoint: *mut c_void,
    SizeOfImage: u32,
    FullDllName: UNICODE_STRING,
    BaseDllName: UNICODE_STRING,
}

#[cfg(target_arch = "x86_64")]
unsafe fn get_peb() -> *mut PEB {
    let peb: *mut PEB;
    std::arch::asm!("mov {}, gs:[0x60]", out(reg) peb);
    peb
}

#[cfg(target_arch = "x86")]
unsafe fn get_peb() -> *mut PEB {
    let peb: *mut PEB;
    std::arch::asm!("mov {}, fs:[0x30]", out(reg) peb);
    peb
}

fn sdbm_hash(s: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for &c in s {
        hash = (c as u32).wrapping_add(hash.wrapping_shl(6)).wrapping_add(hash.wrapping_shl(16)).wrapping_sub(hash);
    }
    hash
}

fn sdbm_hash_lower(s: &[u8]) -> u32 {
    let mut hash: u32 = 0;
    for &c in s {
        let c_lower = if c >= b'A' && c <= b'Z' { c + 32 } else { c };
        hash = (c_lower as u32).wrapping_add(hash.wrapping_shl(6)).wrapping_add(hash.wrapping_shl(16)).wrapping_sub(hash);
    }
    hash
}

unsafe fn get_module_handle_custom_by_hash(module_hash: u32) -> isize {
    let peb = get_peb();
    if peb.is_null() || (*peb).Ldr.is_null() {
        return 0;
    }

    let ldr = &*(*peb).Ldr;
    let head = &ldr.InLoadOrderModuleList as *const LIST_ENTRY;
    let mut current = (*head).Flink;

    while current != head as *mut LIST_ENTRY {
        let entry = current as *mut LDR_DATA_TABLE_ENTRY;
        
        let base_dll_name = &(*entry).BaseDllName;
        if !base_dll_name.Buffer.is_null() {
            let len = base_dll_name.Length as usize / 2;
            let slice = std::slice::from_raw_parts(base_dll_name.Buffer, len);
            let dll_name_w = String::from_utf16_lossy(slice);
            
            if sdbm_hash_lower(dll_name_w.as_bytes()) == module_hash {
                return (*entry).DllBase as isize;
            }
        }

        current = (*current).Flink;
    }

    0
}

pub unsafe fn load_library(dll_name: &[u8]) -> Result<isize, String> {
    use windows_sys::Win32::System::LibraryLoader::LoadLibraryA;
    use rsl_macros::obfuscation_noise_macro;
    use obfstr::obfstr;

    let name_str = String::from_utf8_lossy(dll_name);
    let name_trimmed = name_str.trim_matches(char::from(0));
    let module_hash = sdbm_hash_lower(name_trimmed.as_bytes());

    let dll = get_module_handle_custom_by_hash(module_hash);
    if dll != 0 {
        obfuscation_noise_macro!();
		#[cfg(feature = "debug")]
		crate::utils::print_message(&format!("{} {}", obfstr!("Get module handle by custom GetModuleHandle:"), name_trimmed));
        Ok(dll)
    } else {
        let dll = LoadLibraryA(dll_name.as_ptr() as *const u8);
        if dll == 0 {
            Err(obfstr!("LoadLibraryA failed").to_string())
        } else {
            obfuscation_noise_macro!();
			#[cfg(feature = "debug")]
			crate::utils::print_message(&format!("{} {}", obfstr!("Module loaded by LoadLibraryA:"), name_trimmed));
            Ok(dll)
        }
    }
}

#[repr(C)]
struct IMAGE_DOS_HEADER {
    e_magic: u16,
    e_cblp: u16,
    e_cp: u16,
    e_crlc: u16,
    e_cparhdr: u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss: u16,
    e_sp: u16,
    e_csum: u16,
    e_ip: u16,
    e_cs: u16,
    e_lfarlc: u16,
    e_ovno: u16,
    e_res: [u16; 4],
    e_oemid: u16,
    e_oeminfo: u16,
    e_res2: [u16; 10],
    e_lfanew: i32,
}

#[repr(C)]
struct IMAGE_DATA_DIRECTORY {
    VirtualAddress: u32,
    Size: u32,
}

#[repr(C)]
struct IMAGE_EXPORT_DIRECTORY {
    Characteristics: u32,
    TimeDateStamp: u32,
    MajorVersion: u16,
    MinorVersion: u16,
    Name: u32,
    Base: u32,
    NumberOfFunctions: u32,
    NumberOfNames: u32,
    AddressOfFunctions: u32,
    AddressOfNames: u32,
    AddressOfNameOrdinals: u32,
}

unsafe fn get_proc_address_custom_by_hash(module_handle: isize, proc_hash: u32) -> *const () {
    let base = module_handle as *const u8;
    let dos_header = &*(base as *const IMAGE_DOS_HEADER);
    
    if dos_header.e_magic != 0x5A4D {
        return std::ptr::null();
    }

    let nt_offset = dos_header.e_lfanew as usize;
    let nt_headers_ptr = base.add(nt_offset);
    
    let signature = *(nt_headers_ptr as *const u32);
    if signature != 0x00004550 {
        return std::ptr::null();
    }

    let optional_header_ptr = nt_headers_ptr.add(24);
    let magic = *(optional_header_ptr as *const u16);
    
    let export_dir_rva = if magic == 0x20B {
        let data_dir_ptr = optional_header_ptr.add(112) as *const IMAGE_DATA_DIRECTORY;
        (*data_dir_ptr).VirtualAddress
    } else if magic == 0x10B {
        let data_dir_ptr = optional_header_ptr.add(96) as *const IMAGE_DATA_DIRECTORY;
        (*data_dir_ptr).VirtualAddress
    } else {
        return std::ptr::null();
    };

    if export_dir_rva == 0 {
        return std::ptr::null();
    }

    let export_dir = &*(base.add(export_dir_rva as usize) as *const IMAGE_EXPORT_DIRECTORY);
    let number_of_names = export_dir.NumberOfNames;
    let address_of_names = base.add(export_dir.AddressOfNames as usize) as *const u32;
    let address_of_name_ordinals = base.add(export_dir.AddressOfNameOrdinals as usize) as *const u16;
    let address_of_functions = base.add(export_dir.AddressOfFunctions as usize) as *const u32;

    for i in 0..number_of_names {
        let name_rva = *address_of_names.add(i as usize);
        let name_ptr = base.add(name_rva as usize);
        let name_c_str = std::ffi::CStr::from_ptr(name_ptr as *const i8);
        
        if let Ok(name) = name_c_str.to_str() {
            if sdbm_hash(name.as_bytes()) == proc_hash {
                let ordinal = *address_of_name_ordinals.add(i as usize);
                let func_rva = *address_of_functions.add(ordinal as usize);
                return base.add(func_rva as usize) as *const ();
            }
        }
    }

    std::ptr::null()
}

pub unsafe fn get_proc_address(dll: isize, name: &[u8]) -> Result<*const (), String> {
    use windows_sys::Win32::System::LibraryLoader::GetProcAddress;
    use obfstr::obfstr;
    use rsl_macros::obfuscation_noise_macro;

    let name_str = String::from_utf8_lossy(name);
    let name_trimmed = name_str.trim_matches(char::from(0));
    let proc_hash = sdbm_hash(name_trimmed.as_bytes());

    let addr = get_proc_address_custom_by_hash(dll, proc_hash);
    if !addr.is_null() {
        obfuscation_noise_macro!();
        #[cfg(feature = "debug")]
        crate::utils::print_message(&format!("{} {}", obfstr!("Get proc address by custom GetProcAddress:"), name_trimmed));
        Ok(addr)
    } else {
        let addr = GetProcAddress(dll, name.as_ptr() as *const u8);
        if let Some(f) = addr {
            obfuscation_noise_macro!();
            #[cfg(feature = "debug")]
            crate::utils::print_message(&format!("{} {}", obfstr!("Get proc address by GetProcAddress:"), name_trimmed));
            Ok(f as *const ())
        } else {
            Err(obfstr!("GetProcAddress failed").to_string())
        }
    }
}