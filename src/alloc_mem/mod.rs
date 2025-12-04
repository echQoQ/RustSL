#[cfg(feature = "alloc_mem_va")]
mod va;
#[cfg(feature = "alloc_mem_va")]
pub use crate::alloc_mem::va::alloc_mem;

#[cfg(feature = "alloc_mem_global")]
mod global;
#[cfg(feature = "alloc_mem_global")]
pub use crate::alloc_mem::global::alloc_mem;

#[cfg(feature = "alloc_mem_local")]
mod local;
#[cfg(feature = "alloc_mem_local")]
pub use crate::alloc_mem::local::alloc_mem;

#[cfg(feature = "alloc_mem_mapview")]
mod mapview;
#[cfg(feature = "alloc_mem_mapview")]
pub use crate::alloc_mem::mapview::alloc_mem;

#[cfg(feature = "alloc_mem_heap")]
mod heap;
#[cfg(feature = "alloc_mem_heap")]
pub use crate::alloc_mem::heap::alloc_mem;

#[cfg(feature = "alloc_mem_section")]
mod section;
#[cfg(feature = "alloc_mem_section")]
pub use crate::alloc_mem::section::alloc_mem;

#[cfg(feature = "alloc_mem_sh_alloc")]
mod sh_alloc;
#[cfg(feature = "alloc_mem_sh_alloc")]
pub use crate::alloc_mem::sh_alloc::alloc_mem;

#[cfg(feature = "alloc_mem_snmp_util")]
mod snmp_util;
#[cfg(feature = "alloc_mem_snmp_util")]
pub use crate::alloc_mem::snmp_util::alloc_mem;

#[cfg(feature = "alloc_mem_va_from_app")]
mod va_from_app;
#[cfg(feature = "alloc_mem_va_from_app")]
pub use crate::alloc_mem::va_from_app::alloc_mem;