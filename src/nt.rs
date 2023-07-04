#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use winapi::shared::ntdef::PHYSICAL_ADDRESS;

pub type POOL_TYPE = i32;

pub const PagedPool: POOL_TYPE = 1;
pub const NonPagedPool: POOL_TYPE = 0;
pub const NonPagedPoolExecute: POOL_TYPE = 0;
pub const NonPagedPoolMustSucceed: POOL_TYPE = 2;
pub const DontUseThisType: POOL_TYPE = 3;
pub const NonPagedPoolCacheAligned: POOL_TYPE = 4;
pub const PagedPoolCacheAligned: POOL_TYPE = 5;
pub const NonPagedPoolCacheAlignedMustS: POOL_TYPE = 6;
pub const MaxPoolType: POOL_TYPE = 7;
pub const NonPagedPoolBase: POOL_TYPE = 0;
pub const NonPagedPoolBaseMustSucceed: POOL_TYPE = 2;
pub const NonPagedPoolBaseCacheAligned: POOL_TYPE = 4;
pub const NonPagedPoolBaseCacheAlignedMustS: POOL_TYPE = 6;
pub const NonPagedPoolSession: POOL_TYPE = 32;
pub const PagedPoolSession: POOL_TYPE = 33;
pub const NonPagedPoolMustSucceedSession: POOL_TYPE = 34;
pub const DontUseThisTypeSession: POOL_TYPE = 35;
pub const NonPagedPoolCacheAlignedSession: POOL_TYPE = 36;
pub const PagedPoolCacheAlignedSession: POOL_TYPE = 37;
pub const NonPagedPoolCacheAlignedMustSSession: POOL_TYPE = 38;
pub const NonPagedPoolNx: POOL_TYPE = 512;
pub const NonPagedPoolNxCacheAligned: POOL_TYPE = 516;
pub const NonPagedPoolSessionNx: POOL_TYPE = 544;

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn ExAllocatePool(pool_type: POOL_TYPE, number_of_bytes: usize) -> *mut u64;
    pub fn ExAllocatePoolWithTag(
        pool_type: POOL_TYPE, number_of_bytes: usize, tag: u32,
    ) -> *mut u64;
    pub fn ExFreePool(pool: u64);
    pub fn MmAllocateContiguousMemorySpecifyCacheNode(
        NumberOfBytes: usize, LowestAcceptableAddress: PHYSICAL_ADDRESS,
        HighestAcceptableAddress: PHYSICAL_ADDRESS, BoundaryAddressMultiple: PHYSICAL_ADDRESS,
        CacheType: MEMORY_CACHING_TYPE, PreferredNode: NODE_REQUIREMENT,
    ) -> *mut u64;
    pub fn MmFreeContiguousMemory(BaseAddress: *mut u64);
}

pub const MM_ANY_NODE_OK: u32 = 0x80000000;
pub type NODE_REQUIREMENT = u32;

#[repr(i32)]
pub enum MEMORY_CACHING_TYPE {
    MmNonCached = 0,
    MmCached = 1,
    MmWriteCombined = 2,
    MmHardwareCoherentCached = 3,
    MmNonCachedUnordered = 4,
    MmUSWCCached = 5,
    MmMaximumCacheType = 6,
    MmNotMapped = -1,
}
