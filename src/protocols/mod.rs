// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

pub mod console;
pub mod device_path;
pub mod graphics_output;
pub mod file_system;

//* Use from local library *//
use utility::Guid;

pub trait Protocol {
    const GUID: Guid;
}