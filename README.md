![Rust](https://github.com/not-matthias/kernel-alloc-rs/workflows/Rust/badge.svg)
[![crates.io](https://img.shields.io/crates/v/kernel-alloc.svg)](https://crates.io/crates/kernel-alloc)
[![docs.rs](https://docs.rs/kernel-alloc/badge.svg)](https://docs.rs/kernel-alloc)

# kernel-alloc-rs

## Why?

Rust has many useful abstractions and utils that require heap allocations. `String`, `Vec` and `Box` are some of them. To be able to use them, we need to allocate memory at runtime, which requires a custom allocator. 

If you want to find out more about it, please refer to the [alloc::GlobalAllocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) or [alloc::Allocator](https://doc.rust-lang.org/std/alloc/trait.Allocator.html) and the Rust book for [global_allocator](https://doc.rust-lang.org/1.26.2/unstable-book/language-features/global-allocator.html) or [allocator_api](https://doc.rust-lang.org/1.26.2/unstable-book/library-features/allocator-api.html). 

## Example

Add the following to your code to define new global allocator: 

```rust
use kernel_alloc::KernelAlloc;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;
```

Add the following to your code to define new physical allocator: 

```rust
use kernel_alloc::PhysicalAllocator;
```
