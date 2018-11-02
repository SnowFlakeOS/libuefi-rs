// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

// See also http://wiki.phoenix.com/wiki/index.php/EFI_TIME

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(C)]
pub struct Time {
    /* --- The current local date --- */
    /// Range: 1998 ~ 20XX
    year: u16,
    /// Range: 1 ~ 12
    month: u8,
    /// Range: 1 ~ 31
    day: u8,

    /*
        --- The current local time. 
        Nanoseconds report the current fraction of a second in the device. 
        The format of the time is hh:mm:ss.nnnnnnnnn. 
        A battery backed real time clock device maintains the date and time.
        TimeZone The time's offset in minutes from GMT. 
        If the value is EFI_UNSPECIFIED_TIMEZONE, then the time is interpreted as a local time. --- 
    */
    /// Range: 0 ~ 23
    hour: u8,
    /// Range: 0 ~ 59
    minute: u8,
    /// Range: 0 ~ 59
    second: u8,
    _pad1: u8,
    /// Range: 0 ~ 999,999,999
    nano_second: u32,
    /// Range: -1440 to 1440 or 2047
    time_zone: TimeZone,

    /// A bitmask containing the daylight savings time information for the time.
    day_light: Daylight,
    _pad2: u8
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i16)]
pub enum TimeZone {
    Unspecified = 0x07FF
}

impl Default for TimeZone {
    fn default() -> Self {
        TimeZone::Unspecified
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Daylight {
    Adjust = 0x01,
    In = 0x02
}

impl Default for Daylight {
    fn default() -> Self {
        Daylight::Adjust
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
#[repr(C)]
pub struct TimeCapablities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool
}

impl Time {
    pub fn format_time(&self) -> &str { // Unused
        ""
    }

    pub fn year(&self) -> u16 {
        (self.year)
    }

    pub fn month(&self) -> u8 {
        (self.month)
    }

    pub fn day(&self) -> u8 {
        (self.day)
    }
}