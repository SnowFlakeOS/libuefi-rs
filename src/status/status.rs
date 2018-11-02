//! Some code was borrowed from [uefi-rs](https://github.com/GabrielMajeri/uefi-rs/blob/master/src/error/status.rs)

//* Use from external library *//
use core::ops;

//* Use from local library *//
use super::{Result, Completion};

//* Constants & Types *//
const HIGHEST_BIT_SET: usize = !((!0_usize) >> 1);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(usize)]
pub enum Status {
    Success,
    LoadError,
    InvalidParameter,
    Unsupported,
    BadBufferSize,
    BufferTooSmall,
    NotReady,
    DeviceError,
    WriteProtected,
    OutOfResources,
    VolumeCorrupted,
    VolumeFull,
    NoMedia,
    MediaChanged,
    NotFound,
    AccessDenied,
    NoResponse,
    NoMapping,
    Timeout,
    NotStarted,
    AlreadyStarted,
    Aborted,
    IcmpError,
    TftpError,
    ProtocolError,
    IncompatibleVersion,
    SecurityViolation,
    CrcError,
    EndOfMedia,
    Error29,
    Error30,
    EndOfFile,
    InvalidLanguage,
    CompromisedData,
    Error34,
    HttpError,
    Unknown
}

impl Status {
    #[inline]
    pub fn is_success(self) -> bool {
        self == Status::Success
    }

    #[inline]
    pub fn is_warning(self) -> bool {
        (self as usize) & HIGHEST_BIT_SET == 0
    }

    #[inline]
    pub fn is_error(self) -> bool {
        (self as usize) & HIGHEST_BIT_SET != 0
    }

    #[inline]
    pub fn into_with<T, F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> T,
    {
        if self.is_success() {
            Ok(Completion::Success(f()))
        } else if self.is_warning() {
            Ok(Completion::Warning(f(), self))
        } else {
            Err(self)
        }
    }
}

impl Into<Result<()>> for Status {
    #[inline]
    fn into(self) -> Result<()> {
        self.into_with(|| ())
    }
}

impl ops::Try for Status {
    type Ok = Completion<()>;
    type Error = Status;

    fn into_result(self) -> Result<()> {
        self.into()
    }

    fn from_error(error: Self::Error) -> Self {
        error
    }

    fn from_ok(_: Self::Ok) -> Self {
        Status::Success
    }
}