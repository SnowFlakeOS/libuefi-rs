// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from local library *//
use utility::status::{Status, Result};

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct TextOutputMode {
    pub max_mode: i32,
    pub ode: i32,
    pub attribute: i32,
    pub cursor_x: i32,
    pub cursor_y: i32,
    pub cursor_visible: bool,
}

#[repr(C)]
pub struct TextOutput {
    reset: efi_fcn! { fn(&TextOutput, bool) -> Status },
    output_string: efi_fcn! { fn(&TextOutput, *const u16) -> Status },
    test_string: efi_fcn! { fn(&TextOutput, *const u16) -> Status },
    query_mode: efi_fcn! { fn(&TextOutput, usize, &mut usize, &mut usize) -> Status },
    set_mode: efi_fcn! { fn(&TextOutput, usize) -> Status },
    set_attribute: efi_fcn! { fn(&TextOutput, usize) -> Status },
    clear_screen: efi_fcn! { fn(&TextOutput) -> Status },
    set_cursor_position: efi_fcn! { fn(&TextOutput, usize, usize) -> Status },
    enable_cursor: efi_fcn! { fn(&TextOutput, bool) -> Status },
    pub mode: &'static TextOutputMode,
}

impl TextOutput {
    pub fn reset(&mut self, extended: bool) -> Result<()> {
        unsafe { (self.reset)(self, extended).into() }
    }

    pub fn output_string(&mut self, string: *const u16) -> Result<()> {
        unsafe { (self.output_string)(self, string).into() }
    }

    pub fn test_string(&mut self, string: *const u16) -> bool {
        unsafe { 
            match (self.test_string)(self, string) {
                Status::Success => true,
                _ => false
            }
        }
    }

    pub fn query_mode(&mut self, index: usize) -> Result<(usize, usize)> {
        let (mut columns, mut rows) = (0, 0);
        unsafe { (self.query_mode)(self, index, &mut columns, &mut rows) };
        Ok((columns, rows))
    }

    pub fn set_mode(&mut self, index: usize) -> Result<()> {
        unsafe { (self.set_mode)(self, index).into() }
    }


    pub fn set_attribute(&mut self, attr: usize) -> Result<()> {
        unsafe { (self.set_attribute)(self, attr).into() }
    }

    pub fn clear_screen(&mut self) -> Result<()> {
        unsafe { (self.clear_screen)(self).into() }
    }

    pub fn set_cursor_position(&mut self, x: usize, y: usize) -> Result<()> {
        unsafe { (self.set_cursor_position)(self, x, y).into() }
    }

    pub fn enable_cursor(&mut self, visible: bool) -> Result<()> {
        unsafe { (self.enable_cursor)(self, visible).into() }
    }
}