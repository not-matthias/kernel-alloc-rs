use winapi::shared::{
    basetsd::SIZE_T,
    ntdef::{PHYSICAL_ADDRESS, PVOID},
};

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
    pub fn MmAllocateContiguousMemorySpecifyCacheNode(
        NumberOfBytes: SIZE_T, LowestAcceptableAddress: PHYSICAL_ADDRESS, HighestAcceptableAddress: PHYSICAL_ADDRESS,
        BoundaryAddressMultiple: PHYSICAL_ADDRESS, CacheType: MEMORY_CACHING_TYPE, PreferredNode: NODE_REQUIREMENT,
    ) -> PVOID;
    pub fn MmFreeContiguousMemory(BaseAddress: PVOID);
}

pub const MM_ANY_NODE_OK: u32 = 0x80000000;
#[allow(non_camel_case_types)]
pub type NODE_REQUIREMENT = u32;

#[repr(C)]
pub enum MEMORY_CACHING_TYPE {
    MmNonCached = 0,
    MmCached = 1,
    MmWriteCombined = 2,
    MmHardwareCoherentCached,
    MmNonCachedUnordered,
    MmUSWCCached,
    MmMaximumCacheType,
    MmNotMapped = -1,
}
