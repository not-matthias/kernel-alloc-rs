//! Rust has many useful abstractions and utils that require heap allocations.
//! `String`, `Vec` and `Box` are some of them. To be able to use them, we need
//! to allocate memory at runtime, which requires a custom allocator.
//!
//! If you want to find out more about it, please refer to the [alloc::GlobalAllocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) or [alloc::Allocator](https://doc.rust-lang.org/std/alloc/trait.Allocator.html) and the Rust book for [global_allocator](https://doc.rust-lang.org/1.26.2/unstable-book/language-features/global-allocator.html) or [allocator_api](https://doc.rust-lang.org/1.26.2/unstable-book/library-features/allocator-api.html).
//!
//! ## Example
//!
//! Add the following to your code to define a new global allocator:
//!
//! ```rust
//! use kernel_alloc::KernelAlloc;
//!
//! #[global_allocator]
//! static GLOBAL: KernelAlloc = KernelAlloc;
//! ```
//!
//! ## Example
//!
//! Add the following to your code to define a new physical allocator:
//!
//! ```rust
//! use kernel_alloc::PhysicalAllocator;
//!
//! #[global_allocator]
//! static GLOBAL: PhysicalAllocator = PhysicalAllocator;
//! ```

#![no_std]
#![feature(alloc_error_handler)]
#![feature(allocator_api)]

extern crate alloc;

use crate::nt::{ExFreePool, MEMORY_CACHING_TYPE::MmCached};
use alloc::alloc::handle_alloc_error;
use core::{
    alloc::{AllocError, Allocator, GlobalAlloc, Layout},
    ptr::NonNull,
};
use nt::{
    MmAllocateContiguousMemorySpecifyCacheNode, MmFreeContiguousMemory, NonPagedPool,
    MM_ANY_NODE_OK,
};
use winapi::shared::ntdef::PHYSICAL_ADDRESS;

#[doc(hidden)] pub mod nt;

#[cfg(feature = "pool-tag")]
const POOL_TAG: u32 = u32::from_ne_bytes(*b"tsuR");

/// The global kernel allocator structure.
pub struct KernelAlloc;

/// The physical kernel allocator structure.
pub struct PhysicalAllocator;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(feature = "pool-tag")]
        let pool = nt::ExAllocatePoolWithTag(NonPagedPool, layout.size(), POOL_TAG);

        #[cfg(not(feature = "pool-tag"))]
        let pool = nt::ExAllocatePool(NonPagedPool, layout.size());
        if pool.is_null() {
            handle_alloc_error(layout);
        }

        pool as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        ExFreePool(ptr as _);
    }
}

unsafe impl Allocator for PhysicalAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let mut boundary: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };
        let mut lowest: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };
        let mut highest: PHYSICAL_ADDRESS = unsafe { core::mem::zeroed() };

        unsafe { *(boundary.QuadPart_mut()) = 0 };
        unsafe { *(lowest.QuadPart_mut()) = 0 };
        unsafe { *(highest.QuadPart_mut()) = -1 };

        let memory = unsafe {
            MmAllocateContiguousMemorySpecifyCacheNode(
                layout.size(),
                lowest,
                highest,
                boundary,
                MmCached,
                MM_ANY_NODE_OK,
            )
        } as *mut u8;
        if memory.is_null() {
            Err(AllocError)
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(memory, layout.size()) };
            Ok(unsafe { NonNull::new_unchecked(slice) })
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        MmFreeContiguousMemory(ptr.cast().as_ptr());
    }
}

unsafe impl Allocator for KernelAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let memory = unsafe { nt::ExAllocatePool(NonPagedPool, layout.size()) } as *mut u8;
        if memory.is_null() {
            Err(AllocError)
        } else {
            let slice = unsafe { core::slice::from_raw_parts_mut(memory, layout.size()) };
            Ok(unsafe { NonNull::new_unchecked(slice) })
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, _layout: Layout) {
        ExFreePool(ptr.cast().as_ptr() as *mut u64 as _);
    }
}
