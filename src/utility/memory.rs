// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from local library *//
use super::{PoolPointer, Void};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PhysAddress(u64);

impl PhysAddress {
    pub fn new(address: u64) -> Self{
        PhysAddress(address)
    }

    pub fn from_mut_u8(ptr: PoolPointer<u8>) -> Self {
        PhysAddress(ptr as u64)
    }

    pub fn from_ptr(ptr: PoolPointer<Void>) -> Self {
        PhysAddress(ptr as u64)
    }

    pub fn get(&self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VirtAddress(u64);

// See also http://wiki.phoenix.com/wiki/index.php/EFI_MEMORY_DESCRIPTOR

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct MemoryDescriptor {
    mem_type: MemoryType,
    physical_start: PhysAddress,
    virtual_start: VirtAddress,
    count: u64, // Pages
    attribute: MemoryAttribute
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u64)]
pub enum MemoryAttribute {
    ///
    /// Memory cacheability attribute: 
    ///

    /// Being configured as not cacheable. 
    EfiMemoryUC = 0x0000000000000001,
    /// Being configured as write combining.  
    EfiMemoryWC = 0x0000000000000002,
    /// Being configured as cacheable with a “write through” policy. 
    /// Writes that hit in the cache will also be written to main memory. 
    EfiMemoryWT = 0x0000000000000004, 
    /// Being configured as cacheable with a “write back” policy. 
    /// Reads and writes that hit in the cache do not propagate to main memory. 
    /// Dirty data is written back to main memory when a new cache line is allocated. 
    EfiMemoryWB = 0x0000000000000008, 
    /// Being configured as not cacheable, exported, and supports the “fetch and add” semaphore mechanism. 
    EfiMemoryUCE = 0x0000000000000010,

    ///
    /// Physical memory protection attribute:
    /// 
    
    /// Being configured as write-protected by system hardware. 
    EfiMemoryWP = 0x0000000000001000,
    /// Being configured as read-protected by system hardware. 
    EfiMemoryRP = 0x0000000000002000,
    /// Being configured so it is protected by system hardware from executing code.  
    EfiMemoryXP = 0x0000000000004000,

    ///
    /// Runtime memory attribute:
    /// 

    /// The memory region needs to be given a virtual mapping by the operating system when SetVirtualAddressMap() is called. 
    EfiMemoryRuntime = 0x8000000000000000
}

// See also http://wiki.phoenix.com/wiki/index.php/EFI_MEMORY_TYPE

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum MemoryType {
    /// Not used.
    EfiReservedMemoryType, 
    /// The code portions of a loaded application. 
    /// 
    /// (Note that UEFI OS loaders are UEFI applications.)
    EfiLoaderCode, 
    /// The data portions of a loaded application and the default data allocation type used by an application to allocate pool memory. 
    EfiLoaderData,
    /// The code portions of a loaded Boot Services Driver. 
    EfiBootServicesCode,
    /// The data portions of a loaded Boot Serves Driver, and the default data allocation type used by a Boot Services Driver to allocate pool memory. 
    EfiBootServicesData,
    /// The code portions of a loaded Runtime Services Driver. 
    EfiRuntimeServicesCode,
    /// The data portions of a loaded Runtime Services Driver and the default data allocation type used by a Runtime Services Driver to allocate pool memory. 
    EfiRuntimeServicesData,
    /// Free (unallocated) memory.
    EfiConventionalMemory,
    /// Memory in which errors have been detected. 
    EfiUnusableMemory, 
    /// Memory that holds the ACPI tables. 
    EfiACPIReclaimMemory,
    /// Address space reserved for use by the firmware.
    EfiACPIMemoryNVS,
    /// Used by system firmware to request that a memory-mapped IO region be mapped by the OS to a virtual address so it can be accessed by EFI runtime services. 
    EfiMemoryMappedIO,
    /// System memory-mapped IO region that is used to translate memory cycles to IO cycles by the processor. Note: There is only one region of type EfiMemoryMappedIoPortSpace defined in the architecture for Itanium-based platforms. As a result, there should be one and only one region of type EfiMemoryMappedIoPortSpace in the EFI memory map of an Itanium-based platform. 
    EfiMemoryMappedIOPortSpace,
    /// Address space reserved by the firmware for code that is part of the processor. 
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType
}