//! Some code was borrowed from [uefi-rs](https://github.com/GabrielMajeri/uefi-rs/blob/master/src/error/mod.rs)

pub mod status;
pub mod completion;

//* Use from local library *//
pub use self::status::Status;
pub use self::completion::Completion;

pub type Result<T> = core::result::Result<Completion<T>, Status>;

/// Extension trait for Result which helps dealing with UEFI's warnings
pub trait ResultExt<T> {
    /// Treat warnings as errors
    fn warning_as_error(self) -> core::result::Result<T, Status>;
    
    /// Expect success without warnings, panic otherwise
    fn unwrap_success(self) -> T;

    /// Expect success without warnings, panic with provided message otherwise
    fn expect_success(self, msg: &str) -> T;

    /// Transform the inner output, if any
    fn map_inner<U>(self, f: impl Fn(T) -> U) -> Result<U>;
}

impl<T> ResultExt<T> for Result<T> {
    fn warning_as_error(self) -> core::result::Result<T, Status> {
        match self {
            Ok(Completion::Success(v)) => Ok(v),
            Ok(Completion::Warning(_, s)) => Err(s),
            Err(s) => Err(s),
        }
    }

    fn unwrap_success(self) -> T {
        self.unwrap().unwrap()
    }

    fn expect_success(self, msg: &str) -> T {
        self.expect(msg).expect(msg)
    }

    fn map_inner<U>(self, f: impl Fn(T) -> U) -> Result<U> {
        self.map(|completion| completion.map(f))
    }
}