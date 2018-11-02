// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

// See also http://wiki.phoenix.com/wiki/index.php/EFI_GRAPHICS_OUTPUT_PROTOCOL

//* Use from external library *//
use core::{ptr, mem};

//* Use from local library *//
use super::Protocol;
use status::{Result, Status, Completion};
use utility::{Guid, PhysAddress};
use utility::guid::GRAPHICS_OUTPUT_PROTOCOL_GUID;

#[repr(C)]
pub struct GraphicsOutput {
    /// Returns information for an available graphics mode that the graphics device and the set of active video output devices supports.
    query_mode: efi_fcn! { fn(this: &GraphicsOutput, mode_number: u32, size_of_info: &mut usize, info: &mut *const ModeInfo) -> Status },
    /// Set the video device into the specified mode and clears the visible portions of the output display to black.
    set_mode: efi_fcn! { fn(this: &mut GraphicsOutput, mode_number: u32) -> Status },
    /// Blt a rectangle of pixels on the graphics screen. Blt stands for BLock Transfer.
    blt: efi_fcn! { fn(this: &mut GraphicsOutput, blt_buffer: *mut BltPixel, blt_operation: BltOperation, src_x: usize, src_y: usize, des_x: usize, des_y: usize, width: usize, height: usize, delta: usize) -> Status },
    mode: &'static Mode
}

impl Protocol for GraphicsOutput {
    const GUID: Guid = GRAPHICS_OUTPUT_PROTOCOL_GUID;
}

struct ModeIter<'a> {
    gop: &'a GraphicsOutput,
    current: u32,
    max: u32,
}

impl GraphicsOutput {
    pub unsafe fn query_mode(&self, index: u32) -> Result<SimpleMode> {
        let mut info_size = 0;
        let mut info = ptr::null();

        (self.query_mode)(self, index, &mut info_size, &mut info).into_with(|| {
                let info = &*info;
                SimpleMode {
                    index,
                    info_size,
                    info
                }
            }
        )
    }

    pub fn mode(&self) -> &'static Mode {
        (self.mode)
    }

    pub unsafe fn set_mode(&mut self, mode: &SimpleMode) -> Result<()> {
        (self.set_mode)(self, mode.index).into()
    }

	pub unsafe fn blt_fill(&mut self, px: BltPixel, w: usize, h: usize,  x: usize, y: usize) -> Result<()> {
		(self.blt)(self, &px as *const _ as *mut _, BltOperation::EfiBltVideoFill, x, y, 0,0, w, h, 0).into()
	}
	pub unsafe fn blt_to_video(&mut self, data: *mut BltPixel, w: usize, h: usize, x: usize, y: usize) -> Result<()> {
		(self.blt)(self, data, BltOperation::EfiBltBufferToVideo, 0, 0, x, y, w, h, 0).into()
	}
	pub unsafe fn blt_from_video(&mut self, data: &mut [BltPixel], w: usize, h: usize, x: usize, y: usize) -> Result<()> {
		(self.blt)(self, data.as_mut_ptr(), BltOperation::EfiBltVideoToBltBuffer, w, h, x, y, 0, 0, 0).into()
	}
	pub unsafe fn blt_inner_video(&mut self,  w: usize, h: usize, src_x: usize, src_y: usize, dst_x: usize, dst_y: usize) -> Result<()> {
		(self.blt)(self, ptr::null_mut(), BltOperation::EfiBltVideoToVideo, w, h, src_x, src_y, dst_x, dst_y, 0).into()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct ModeInfo {
    pub version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: PixelFormat,
    pixel_info: PixelBitMask,
    pixels_per_scan_line: u32
}

impl ModeInfo {
    pub fn width(&self) -> u32 {
        (self.horizontal_resolution)
    }

    pub fn height(&self) -> u32 {
        (self.vertical_resolution)
    }

    pub fn pixel_format(&self) -> PixelFormat {
        (self.pixel_format)
    }

    pub fn pixel_info(&self) -> PixelBitMask {
        (self.pixel_info)
    }

    pub fn pixels_per_scan_line(&self) -> u32 {
        (self.pixels_per_scan_line)
    }
}

impl Default for ModeInfo {
    fn default() -> Self {
        Self {
            version: 0,
            horizontal_resolution: 0,
            vertical_resolution: 0,
            pixel_format: PixelFormat::PixelRedGreenBlueReserved8BitPerColor,
            pixel_info: PixelBitMask::default(),
            pixels_per_scan_line: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct PixelBitMask {
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    _reserved_mask: u32
}

impl Default for PixelBitMask {
    fn default() -> Self {
        Self {
            red_mask: 0,
            green_mask: 0,
            blue_mask: 0,
            _reserved_mask: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    PixelRedGreenBlueReserved8BitPerColor = 0,
    PixelBlueGreenRedReserved8BitPerColor = 1,
    PixelBitMask = 2,
    PixelBltOnly = 3
}

#[repr(C)]
pub struct BltPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    _reserved: u8,
}

impl BltPixel {
    pub fn new(red: u8, green: u8, blue: u8) -> Self{
        Self {
            blue: blue,
            green: green,
            red: red,
            _reserved: 0
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub enum BltOperation {
    EfiBltVideoFill,
    EfiBltVideoToBltBuffer,
    EfiBltBufferToVideo,
    EfiBltVideoToVideo,
    EfiGraphicsOutputBltOperationMax
}

// See also http://wiki.phoenix.com/wiki/index.php/EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE

pub struct SimpleMode {
    index: u32,
    info_size: usize,
    info: &'static ModeInfo,
}

impl SimpleMode {
    pub fn info(&self) -> &'static ModeInfo {
        (self.info)
    }

    pub fn width(&self) -> u32 {
        self.info().width()
    }

    pub fn height(&self) -> u32 {
        self.info().height()
    }
}

impl Default for SimpleMode {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Mode {
    max_mode: u32,
    mode: u32,
    info: &'static ModeInfo,
    size_of_info: usize,
    frame_buffer_base: PhysAddress,
    frame_buffer_size: usize
}

impl Mode {
    pub fn max_mode(&self) -> u32 {
        (self.max_mode)
    }

    pub fn info(&self) -> &'static ModeInfo {
        (self.info)
    }

    pub fn base(&self) -> PhysAddress {
        (self.frame_buffer_base)
    }
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            max_mode: 0,
            mode: 0,
            info: unsafe { &*(0 as *const ModeInfo) },
            size_of_info: 0,
            frame_buffer_base: PhysAddress::new(0),
            frame_buffer_size: 0
        }
    }
}