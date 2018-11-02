// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use libuefi_rs::protocols::graphics_output::GraphicsOutput;

//* Use from local library *//
use crate::boot_services;
use crate::boot::BootServicesExt;

pub trait GraphicsOutputExt {
    fn new() -> &'static mut Self 
        where Self: Sized;
}

impl GraphicsOutputExt for GraphicsOutput {
    fn new() -> &'static mut Self
        where Self: Sized {
        unsafe { &mut *(boot_services().find_protocol::<GraphicsOutput>().unwrap().as_ptr()) }
    }
}