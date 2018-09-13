// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use core::mem;

//* Use from local library *//
use utility::Event;
use utility::status::{Status, Result};

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TextInputKey {
    pub scan_code: u16,
    pub unicode_char: u16,
}

#[repr(C)]
pub struct TextInput {
    reset: efi_fcn! { fn(&TextInput, bool) -> Status },
    read_key_stroke: efi_fcn! { fn(&TextInput, &mut TextInputKey) -> Status},
    pub wait_for_key: Event,
}

impl TextInput {
    pub fn reset(&mut self, extened: bool) -> Result<()> {
        unsafe { (self.reset)(self, extened).into() }
    }

    pub fn read_key_stroke(&mut self) -> Result<TextInputKey> {
        let mut input = unsafe { mem::uninitialized() };
        unsafe { (self.read_key_stroke)(self, &mut input)? };
        Ok(input)
    }
}