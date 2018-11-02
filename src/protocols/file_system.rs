// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

// See also http://wiki.phoenix.com/wiki/index.php/EFI_SIMPLE_FILE_SYSTEM_PROTOCOL
//! Some code was borrowed from [uefi-rs](https://github.com/GabrielMajeri/uefi-rs)

//* Use from external library *//
use core::mem;

//* Use from local library *//
use super::Protocol;
use utility::{Void, PoolPointer, Time, Guid};
use utility::guid::FILE_SYSTEM_GUID;
use status::{Result, Status};

/// SimpleFileSystem Protocol (With Guid)
#[repr(C)]
pub struct SimpleFileSystem {
    pub reversion: u64,
    open_volume: efi_fcn! { fn(this: &mut SimpleFileSystem, root: &mut PoolPointer<FileStruct>) -> Status }
}

impl Protocol for SimpleFileSystem {
    const GUID: Guid = FILE_SYSTEM_GUID;
}

impl SimpleFileSystem {
    pub unsafe fn open(&mut self) -> Result<File> {
        let mut root = Void::new() as PoolPointer<FileStruct>;
        (self.open_volume)(self, &mut root).into_with(|| File::new(root))
    }
}

/// File Protocol
#[repr(C)]
pub struct FileStruct {
    pub reversion: u64,
    /// Opens a new file relative to the source file’s location.
    open: efi_fcn! { fn(this: &mut FileStruct, new_handle: &mut PoolPointer<FileStruct>, file_name: *const u16, open_mode: FileOpenMode, attributes: FileAttr) -> Status },
    /// Closes a specified file handle. 
    close: efi_fcn! { fn(this: &mut FileStruct) -> Status },
    /// Closes and deletes a file.
    delete: efi_fcn! { fn(this: &mut FileStruct) -> Status },
    /// Reads data from a file.
    read: efi_fcn! { fn(this: &mut FileStruct, buffer_size: &mut usize, buffer: PoolPointer<u8>) -> Status },
    /// Writes data to a file.
    write: efi_fcn! { fn(this: &mut FileStruct, buffer_size: &mut usize, buffer: *const u8) -> Status },
    /// Returns a file’s current position.
    get_position: efi_fcn! { fn(this: &mut FileStruct, position: &mut u64) -> Status },
    set_position: efi_fcn! { fn(this: &mut FileStruct, position: u64) -> Status },
    /// Returns information about a file.
    get_info: efi_fcn! { fn(this: &mut FileStruct, information_type: &Guid, buffer_size: &mut usize, buffer: PoolPointer<u8>) -> Status },
    /// Sets information about a file. 
    set_info: efi_fcn! { fn(this: &mut FileStruct, information_type: &Guid, buffer_size: usize, buffer: *const u8) -> Status },
    /// Flushes all modified data associated with a file to a device.
    flush: efi_fcn! { fn(this: &mut FileStruct) -> Status }
}

pub struct File(pub &'static mut FileStruct);

impl File {
    pub unsafe fn new(ptr: *mut FileStruct) -> Self {
        File(&mut *ptr)
    }

    pub unsafe fn inner_open(&mut self, file_name: &[u16], open_mode: FileOpenMode, attr: FileAttr) -> Result<File> {
        let mut new_handle = Void::new() as PoolPointer<FileStruct>;
        (self.0.open)(self.0, &mut new_handle, file_name.as_ptr(), open_mode, attr).into_with(|| File::new(new_handle))
    }

    pub unsafe fn close(self) -> Result<()> {
        (self.0.close)(self.0).into()
    }

    pub unsafe fn delete(self) -> Result<()> {
        let status = (self.0.delete)(self.0).into();
        mem::forget(self);
        status
    }

    pub unsafe fn inner_read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        let mut buffer_size = buffer.len();
        (self.0.read)(self.0, &mut buffer_size, buffer.as_mut_ptr()).into_with(|| buffer_size)
    }

    pub unsafe fn inner_write(&mut self, buffer: &[u8]) -> Result<usize> {
        let mut buffer_size = buffer.len();
        (self.0.write)(self.0, &mut buffer_size, buffer.as_ptr()).into_with(|| buffer_size)
    }

    pub unsafe fn get_position(&mut self) -> Result<u64> {
        let mut pos: u64 = 0;
        (self.0.get_position)(self.0, &mut pos).into_with(|| pos)
    }

    pub unsafe fn set_position(&mut self, pos: u64) -> Result<()> {
        (self.0.set_position)(self.0, pos).into()
    }

    pub unsafe fn flush(&mut self) -> Result<()> {
        (self.0.flush)(self.0).into()
    }
}

/// The mode to open the file. 
/// The only valid combinations that the file may be opened with are: Read, Read/Write, or Create/Read/Write.
#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum FileOpenMode {
    Read = 0x0000000000000001,
    Write = 0x0000000000000002,
    Create = 0x8000000000000000
}

/// Only valid for EFI_FILE_MODE_CREATE, in which case these are the attribute bits for the newly created file.
#[derive(Debug, Copy, Clone)]
#[repr(u64)]
pub enum FileAttr {
    Normal = 0x0000000000000000,
    ReadOnly = 0x0000000000000001,
    Hidden = 0x0000000000000002,
    System = 0x0000000000000004,
    Reserved = 0x0000000000000008,
    Directory = 0x0000000000000010,
    Archive = 0x0000000000000020,
    ValidAttr = 0x0000000000000037
}

#[repr(C)]
pub struct FileInfo {
    pub size: u64,
    pub file_size: u64,
    pub physical_size: u64,
    pub create_time: Time,
    pub last_access_time: Time,
    pub modification_time: Time,
    pub attribute: u64,
    pub file_name: [u16; 256],
}

impl Default for FileInfo {
    fn default() -> Self {
        FileInfo {
            size: Default::default(),
            file_size: Default::default(),
            physical_size: Default::default(),
            create_time: Default::default(),
            last_access_time: Default::default(),
            modification_time: Default::default(),
            attribute: Default::default(),
            file_name: [0; 256],
        }
    }
}