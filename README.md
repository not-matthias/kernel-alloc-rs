![Rust](https://github.com/not-matthias/kernel-alloc-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/kernel-alloc.svg)](https://crates.io/crates/kernel-alloc)
[![docs.rs](https://docs.rs/kernel-alloc/badge.svg)](https://docs.rs/kernel-alloc)

# kernel-alloc-rs

A custom memory allocator tailored for the Windows kernel space.

## Why?

Rust has many useful abstractions and utils that require heap allocations, such as `String`, `Vec`, and `Box`. To be able to use them in the Windows kernel space, we need to allocate memory at runtime, which requires a custom allocator. This crate provides such allocators tailored for the Windows kernel.

For more information on custom allocators in Rust, refer to the [alloc::GlobalAllocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) and [alloc::Allocator](https://doc.rust-lang.org/std/alloc/trait.Allocator.html) documentation. Additionally, the Rust book provides details on [global_allocator](https://doc.rust-lang.org/1.26.2/unstable-book/language-features/global-allocator.html) and [allocator_api](https://doc.rust-lang.org/1.26.2/unstable-book/library-features/allocator-api.html).

## Example

To use `KernelAlloc` or `PhysicalAllocator` as your global allocator, add the appropriate code to your kernel module:

For `KernelAlloc`:

```rust
use kernel_alloc::KernelAlloc;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;
```

For `PhysicalAllocator`:

```rust
use kernel_alloc::PhysicalAllocator;

#[global_allocator]
static GLOBAL: PhysicalAllocator = PhysicalAllocator;
```

## Using with `Box`

Once you've set up `KernelAlloc` or `PhysicalAllocator` as your global allocator, you can use `Box` and other heap-allocated types just like you would in a standard Rust environment.

Here's an example demonstrating how to use both `KernelAlloc` and `PhysicalAllocator` with `Box` to allocate memory for different structs in the Windows kernel:

```rust
use kernel_alloc::{KernelAlloc, PhysicalAllocator};
use core::mem;

pub const PAGE_SIZE: usize = 0x1000;
pub const KERNEL_STACK_SIZE: usize = 0x6000;
pub const STACK_CONTENTS_SIZE: usize = KERNEL_STACK_SIZE - (mem::size_of::<*mut u64>() * 2);

#[repr(C, align(4096))]
pub struct Vmxon {
    pub revision_id: u32,
    pub data: [u8; PAGE_SIZE - 4],
}

#[repr(C, align(4096))]
pub struct HostStackLayout {
    pub stack_contents: [u8; STACK_CONTENTS_SIZE],
    pub padding_1: u64,
    pub reserved_1: u64,
}

pub struct Vmx {
    pub vmxon_region: Box<Vmxon, PhysicalAllocator>,
    pub host_rsp: Box<HostStackLayout, KernelAlloc>,
}

impl Vmx {
    pub fn new() -> Result<Self, AllocError> {
        let vmxon_region = unsafe { Box::try_new_zeroed_in(PhysicalAllocator)?.assume_init() };
        let host_rsp = unsafe { Box::try_new_zeroed_in(KernelAlloc)?.assume_init() };

        Ok(Self {
            vmxon_region: vmxon_region,
            host_rsp: host_rsp,
        })
    }
}
```

## Credits / References

- [Vergilius Project](https://www.vergiliusproject.com/)
- [@not-matthias](https://github.com/not-matthias/)
- @jessiep_
- [@memN0ps](https://github.com/memN0ps/)