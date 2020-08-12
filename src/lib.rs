#![doc(include = "../README.md")]
#![feature(external_doc)]
#![no_std]

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
