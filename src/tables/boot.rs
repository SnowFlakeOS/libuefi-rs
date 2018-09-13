// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//! Some code was borrowed from [uefi-rs](https://github.com/GabrielMajeri/uefi-rs)

//* Use from local library *//
use super::TableHeader;

#[repr(C)]
pub struct BootServices {
    pub hdr: TableHeader,
    
    raise_tpl: efi_fcn! { fn(Tpl) }, // new_tpl
    restore_tpl: efi_fcn! { fn(Tpl) }, // old_tpl
}

#[derive(Debug, Copy, Clone)]
#[repr(usize)]
pub enum Tpl {
    /// Normal task execution level.
    Application = 4,
    /// Async interrupt-style callbacks run at this TPL.
    Callback = 8,
    /// Notifications are masked at this level.
    ///
    /// This is used in critical sections of code.
    Notify = 16,
    /// Highest priority level.
    ///
    /// Even processor interrupts are disable at this level.
    HighLevel = 31,
}