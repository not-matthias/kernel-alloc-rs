#![doc(include = "../README.md")]
#![no_std]
#![feature(external_doc)]
#![feature(alloc_error_handler)]

use crate::nt::{ExAllocatePool, ExFreePool, PoolType};
use core::alloc::{GlobalAlloc, Layout};

#[doc(hidden)] pub mod nt;

/// The global kernel allocator structure.
pub struct KernelAlloc;

unsafe impl GlobalAlloc for KernelAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let pool = ExAllocatePool(PoolType::NonPagedPool, layout.size());

        if pool.is_null() {
            panic!("[kernel-alloc] failed to allocate pool.");
        }

        pool as _
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) { ExFreePool(ptr as _); }
}

#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("{:?} alloc memory error", layout);
}
