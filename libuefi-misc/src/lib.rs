// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(alloc)]
extern crate libuefi_rs;
extern crate libuefi_alloc;
#[macro_use]
extern crate alloc;

#[macro_use]
pub mod marcros;

pub mod io;
pub mod boot;
pub mod protocols;
pub mod string;

//* Use from external library *//
use libuefi_rs::protocols::graphics_output::{GraphicsOutput, SimpleMode};
use libuefi_rs::tables::{BootServices, RuntimeServices, SystemTable};
use libuefi_rs::tables::runtime::ResetType;
use libuefi_rs::status::Status;
use libuefi_rs::protocols::console::output::TextOutput;

//* Constants & Types *//
static mut CONSOLE: Option<*mut TextOutput> = None;
static mut BOOT_SERVICES: Option<&BootServices> = None;
static mut RUNTIME_SERVICES: Option<&RuntimeServices> = None;

pub unsafe fn init(system_table: &'static mut SystemTable) {
    CONSOLE = Some(system_table.console_out as *mut _);
    BOOT_SERVICES = Some(system_table.boot_services());
    RUNTIME_SERVICES = Some(system_table.runtime_services());
    libuefi_alloc::init(boot_services());
}

pub unsafe fn console() -> &'static mut TextOutput {
    &mut *(CONSOLE.unwrap())
}

pub unsafe fn boot_services() -> &'static BootServices {
    BOOT_SERVICES.unwrap()
}

pub unsafe fn runtime_services() -> &'static RuntimeServices {
    RUNTIME_SERVICES.unwrap()
}

pub unsafe fn shutdown() {
    let _ = runtime_services().reset_system(ResetType::EfiResetShutdown, Status::Success);
}

pub unsafe fn set_max_text_mode() {
    let mut max_i = None;
    let mut max_w = 0;
    let mut max_h = 0;

    let output = console();

    for i in 0..(output.mode().max_mode as usize) {
        let (query_mode, status) = output.query_mode(i).unwrap().split();
        if status.is_success() {
            let w = query_mode.0;
            let h = query_mode.1;
            if w >= max_w && h >= max_h {
                max_i = Some(i);
                max_w = w;
                max_h = h;
            }
        }
    }

    if let Some(i) = max_i {
        let _ = output.set_mode(i);
    }
}

pub unsafe fn set_max_graphics_mode(output: &mut GraphicsOutput) {
    let mut max_mode = SimpleMode::default();

    for i in 0..(output.mode().max_mode() as usize) {
        let (mode, status) = output.query_mode(i as u32).unwrap().split();
        if status.is_success() {
            let w = mode.width();
            let h = mode.height();
            if w >= max_mode.width() && h >= max_mode.height() {
                max_mode = mode;
            }
        }
    }

    let _ = output.set_mode(&max_mode);
}

#[panic_handler]
pub extern fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = info.location() {
        print!(
            "Panic in {} at ({}, {}):",
            location.file(),
            location.line(),
            location.column()
        );
        if let Some(message) = info.message() {
            print!(" {}", message);
        }
    }
    loop {}
}

#[alloc_error_handler]
fn out_of_memory(layout: ::core::alloc::Layout) -> ! {
    panic!(
        "Ran out of free memory while trying to allocate {:#?}",
        layout
    );
}