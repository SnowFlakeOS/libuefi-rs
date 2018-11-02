// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from local library *//
use protocols::console::input::TextInput;
use protocols::console::output::TextOutput;
use utility::{Void, Handle};
use super::{TableHeader, BootServices, RuntimeServices};

// See also http://wiki.phoenix.com/wiki/index.php/EFI_SYSTEM_TABLE

#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_reversion: u32,

    console_in_handle: Handle,
    console_in: &'static TextInput,
    console_out_handle: Handle,
    pub console_out: &'static mut TextOutput,
    console_err_handle: Handle,
    console_err: &'static TextOutput,

    runtime_services: &'static RuntimeServices, // Unused
    boot_services: &'static BootServices,

    entries: usize,
    configuration_table: &'static Void, // Unused
}

impl SystemTable {
    pub fn console_in_handle(&self) -> Handle {
        (self.console_in_handle)
    }

    pub fn console_in(&self) -> &'static TextInput {
        (self.console_in)
    }

    pub fn console_out_handle(&self) -> Handle {
        (self.console_out_handle)
    }
    
    pub fn console_err_handle(&self) -> Handle {
        (self.console_err_handle)
    }

    pub fn console_err(&self) -> &'static TextOutput {
        (self.console_err)
    }

    pub fn runtime_services(&self) -> &'static RuntimeServices {
        (self.runtime_services)
    }

    pub fn boot_services(&self) -> &'static BootServices {
        (self.boot_services)
    }

    pub fn entries(&self) -> usize {
        (self.entries)
    }

    pub fn configuration_table(&self) -> &'static Void {
        (self.configuration_table)
    }
}