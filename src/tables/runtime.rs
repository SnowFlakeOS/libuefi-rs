// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

// See also http://wiki.phoenix.com/wiki/index.php/EFI_RUNTIME_SERVICES

//* Use from local library *//
use super::TableHeader;
use status::{Result, Status};
use utility::{Void, Guid, PoolPointer, MemoryDescriptor};
use utility::time::{Time, TimeCapablities};

#[repr(C)]
pub struct RuntimeServices {
    pub hdr: TableHeader,
    
    /* --- Time Services --- */
    pub get_time: efi_fcn! { fn(time: &mut Time, capabilities: Option<&mut TimeCapablities>) -> Status },
    pub set_time: efi_fcn! { fn(time: &Time) -> Status },
    pub get_wakeup_time: efi_fcn! { fn(enabled: &mut bool, pending: &mut bool, time: &mut Time) -> Status },
    pub set_wakeup_time: efi_fcn! { fn(enable: bool, time: Option<&mut Time>) -> Status },

    /* --- Virutal Memory Services --- */
    pub set_virtual_address_map: efi_fcn! { fn(memory_map_size: usize, descriptor_size: usize, descriptor_version: u32, virtual_map: &MemoryDescriptor) -> Status },
    pub convert_pointer: efi_fcn! { fn(debug_disposition: usize, address: &mut *const Void) -> Status },
    
    /* --- Variable Services --- */
    pub get_variable: efi_fcn! { fn(variable_name: &char, vender_guid: &Guid, attributes: Option<&mut u32>, data_size: &mut usize, data: &Void) -> Status },
    pub get_next_variable_name: efi_fcn! { fn(variable_name_size: &mut usize, variable_name: &mut char, vendor_guid: &Guid) -> Status },
    pub set_variable: efi_fcn! { fn(variable_name: &char, vendor_guid: &Guid, attributes: u32, data_size: usize, data: &Void) -> Status },
    pub get_next_high_monotonic_count: efi_fcn! { fn(high_count: &mut u32) -> Status },

    /* --- Other Services --- */
    pub reset_system: efi_fcn! { fn(reset_type: u32, reset_status: Status, data_size: usize, reset_data: PoolPointer<Void>) -> Status }
}

impl RuntimeServices {
    pub unsafe fn reset_system(&self, reset_type: ResetType, reset_status: Status) -> Result<()> {
        (self.reset_system)(reset_type as u32, reset_status, 0, Void::new()).into()
    }
}

#[repr(u32)]
pub enum ResetType {
    EfiResetCold,
    EfiResetWarm,
    EfiResetShutdown
}