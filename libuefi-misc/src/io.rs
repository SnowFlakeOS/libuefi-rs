// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use core::fmt::{self, Write};

//* Use from local library *//
use crate::console;

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, string: &str) -> Result<(), fmt::Error> {
        let _ = unsafe { console().output_string(string) };
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}