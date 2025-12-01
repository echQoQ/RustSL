// alloc_mem: VirtualAlloc implementation
#[cfg(feature = "alloc_mem_va")]
mod va;
#[cfg(feature = "alloc_mem_va")]
pub use crate::alloc_mem::va::alloc_mem;

// alloc_mem: GlobalAlloc implementation
#[cfg(feature = "alloc_mem_global")]
mod global;
#[cfg(feature = "alloc_mem_global")]
pub use crate::alloc_mem::global::alloc_mem;

// alloc_mem: LocalAlloc implementation
#[cfg(feature = "alloc_mem_local")]
mod local;
#[cfg(feature = "alloc_mem_local")]
pub use crate::alloc_mem::local::alloc_mem;

// alloc_mem: MapViewOfFile implementation
#[cfg(feature = "alloc_mem_mapview")]
mod mapview;
#[cfg(feature = "alloc_mem_mapview")]
pub use crate::alloc_mem::mapview::alloc_mem;

// alloc_mem: HeapAlloc implementation
#[cfg(feature = "alloc_mem_heap")]
mod heap;
#[cfg(feature = "alloc_mem_heap")]
pub use crate::alloc_mem::heap::alloc_mem;