#[repr(C)]
pub enum PoolType {
    NonPagedPool,
    NonPagedPoolExecute,
}

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAllocatePool(pool_type: PoolType, number_of_bytes: usize) -> *mut u64;
    pub fn ExAllocatePoolWithTag(pool_type: PoolType, number_of_bytes: usize, tag: u32) -> *mut u64;
    pub fn ExFreePool(pool: u64);
}
