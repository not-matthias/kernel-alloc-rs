![Rust](https://github.com/not-matthias/kernel-alloc-rs/workflows/Rust/badge.svg)

## Why?

Rust has many useful abstractions and utils that require heap allocations. `String`, `Vec` and `Box` are some of them. To be able to use them, we need to allocate memory at runtime, which requires a custom allocator. 

If you want to find out more about it, please refer to the [alloc::GlobalAllocator](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html) and the [Rust book](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/global-allocators.html). 

## Example

Add the following to your code to define new global allocator: 

```rust
use kernel_alloc::KernelAlloc;

#[global_allocator]
static GLOBAL: KernelAlloc = KernelAlloc;
```
