// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from local library *//
use protocols::console::input::TextInput;
use protocols::console::output::TextOutput;
use utility::Handle;
use super::TableHeader;

#[repr(C)]
pub struct SystemTable {
    header: TableHeader,
    pub firmware_vendor: *const u16,
    pub firmware_reversion: u32,
    console_in_handle: Handle,
    console_in: &'static mut TextInput,
    console_out_handle: Handle,
    console_out: &'static mut TextOutput,
    console_err_handle: Handle,
    console_err: &'static mut TextOutput,
}