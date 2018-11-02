// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

pub mod guid; 
pub mod memory;
pub mod time;

//* Use from external library *//
use core::ffi::c_void;

//* Use from local library *//
pub use self::guid::Guid;
pub use self::memory::{PhysAddress, VirtAddress, MemoryType, MemoryDescriptor};
pub use self::time::Time;

pub enum Void {}

impl Void {
    pub fn new() -> PoolPointer<Self> {
        0 as PoolPointer<Self>
    }

    pub fn from_addr(addr: u64) -> PoolPointer<Self> {
        addr as PoolPointer<Self>
    }
}

pub type PoolPointer<T> = *mut T;

pub type EventNotifyFcn = efi_fcn!{ fn(Event, *mut Void) -> () };

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Event(*mut c_void);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Handle(*mut c_void);
