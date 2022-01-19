//! Rust has many useful abstractions and utils that require heap allocations.
//! `String`, `Vec` and `Box` are some of them. To be able to use them, we need
//! to allocate memory at runtime, which requires a custom allocator.
//!
//! If you want to find out more about it, please refer to the [alloc::GlobalAllocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) and the [Rust book](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html).
//!
//! ## Example
//!
//! Add the following to your code to define new global allocator:
//!
//! ```rust
//! use kernel_alloc::KernelAlloc;
//!
//! #[global_allocator]
//! static GLOBAL: KernelAlloc = KernelAlloc;
//! ```

#![no_std]
#![feature(alloc_error_handler)]

extern crate alloc;

use crate::nt::{ExFreePool, PoolType};
use alloc::alloc::handle_alloc_error;
use core::alloc::{GlobalAlloc, Layout};

#[doc(hidden)] pub mod nt;

#[cfg(feature = "pool-tag")]
const POOL_TAG: u32 = u32::from_ne_bytes(*b"tsuR");

/// The global kernel allocator structure.
pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        #[cfg(feature = "pool-tag")]
        let pool = nt::ExAllocatePoolWithTag(PoolType::NonPagedPool, layout.size(), POOL_TAG);

        #[cfg(not(feature = "pool-tag"))]
        let pool = nt::ExAllocatePool(PoolType::NonPagedPool, layout.size());
        if pool.is_null() {
            handle_alloc_error(layout);
        }

        pool as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { ExFreePool(ptr as _); }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("allocation failed: {:?}", layout);
}
