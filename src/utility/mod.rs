// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

#[macro_use] pub mod macros;
pub mod guid; 
pub mod status;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Event(pub usize); // Currently, unused

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Handle(pub usize);