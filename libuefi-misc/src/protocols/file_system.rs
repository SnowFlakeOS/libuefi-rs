// =======================================================================
//  Copyleft SnowFlakeOS Team 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use crate::alloc::vec::Vec;
use libuefi_rs::protocols::file_system::{SimpleFileSystem, File, FileOpenMode, FileAttr};
use libuefi_rs::status::{Result, Status};

//* Use from local library *//
use crate::boot_services;
use crate::boot::BootServicesExt;
use crate::string::wstr;

pub trait SimpleFileSystemExt {
    fn new() -> &'static mut Self 
        where Self: Sized;
}

impl SimpleFileSystemExt for SimpleFileSystem {
    fn new() -> &'static mut Self {
        unsafe { &mut *(boot_services().find_protocol::<SimpleFileSystem>().unwrap().as_ptr()) }
    }
}

pub trait FileExt {
    fn root() -> Result<Self>
        where Self: Sized;
    fn find(path: &str) -> Result<Self>
        where Self: Sized;
    fn load(path: &str) -> Vec<u8>
        where Self: Sized;
    fn open(&mut self, path: &str, mode: FileOpenMode, attr: FileAttr) -> Result<Self>
        where Self: Sized;
    fn read(&mut self, vec: &mut Vec<u8>) -> Result<usize>
        where Self: Sized;
}

impl FileExt for File {
    fn root() -> Result<Self> {
        unsafe { 
            let simple_file_system = SimpleFileSystem::new();
            simple_file_system.open()
        }
    }

    fn find(path: &str) -> Result<Self> {
        let mut root = File::root()?.unwrap();
        match root.open(path, FileOpenMode::Read, FileAttr::Normal) {
            Ok(file) => {
                return Ok(file);
            },
            Err(err) => if err != Status::NotFound {
                return Err(err);
            }
        }
        Err(Status::NotFound)
    }

    fn load(path: &str) -> Vec<u8> {
        let mut file = File::find(path).unwrap().unwrap();
        let mut data: Vec<u8> = vec![];
        let _ = file.read(&mut data);
        data
    }

    fn open(&mut self, path: &str, mode: FileOpenMode, attr: FileAttr) -> Result<Self> {
        let wpath = &wstr(path);
        unsafe { self.inner_open(wpath, mode, attr) }
    }

    fn read(&mut self, vec: &mut Vec<u8>) -> Result<usize> {
        let mut total = 0;
        let mut status = Status::Success;
        
        loop {
            let mut buf = [0; 8192];

            let (size, inner_status) = unsafe { self.inner_read(&mut buf)?.split() };
            status = inner_status;

            if size == 0 {
                break;
            }

            vec.extend(&buf[.. size]);
            total += size;
        }

        status.into_with(|| total)
    }
}