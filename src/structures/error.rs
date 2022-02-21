use winapi::um::errhandlingapi::GetLastError;
use winresult::*;
use std::fmt::{self, Debug, Display, Formatter};



/// An error generated by the `hwnd` crate
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(pub(crate) ErrorHResultOrCode);

impl Error {
    pub(crate) fn new_gle() -> Self {
        Self(ErrorHResultOrCode::from(unsafe { GetLastError() }))
    }

    pub(crate) fn new_gle_nz() -> Result<(), Self> {
        let e = Self::new_gle();
        if e == ERROR::SUCCESS { return Ok(()) }
        Err(e)
    }

    pub const fn to_u32(&self) -> u32 { self.0.to_u32() }
    pub       fn code(&self) -> Option<ErrorCode> { self.0.to_code().map(|c| ErrorCode::from(c)) }
}

impl Debug   for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "hwnd::Error({:?})",           self.0) }}
impl Display for Error { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "error handling HWNDs: {:?}",  self.0) }}

impl PartialEq<ErrorCode            > for Error { fn eq(&self, other: &ErrorCode            ) -> bool { self.to_u32() == other.to_u32() } }
impl PartialEq<HResultError         > for Error { fn eq(&self, other: &HResultError         ) -> bool { self.to_u32() == other.to_u32() } }
impl PartialEq<ErrorHResultOrCode   > for Error { fn eq(&self, other: &ErrorHResultOrCode   ) -> bool { self.to_u32() == other.to_u32() } }

impl PartialEq<Error> for ErrorCode             { fn eq(&self, other: &Error) -> bool { self.to_u32() == other.to_u32() } }
impl PartialEq<Error> for HResultError          { fn eq(&self, other: &Error) -> bool { self.to_u32() == other.to_u32() } }
impl PartialEq<Error> for ErrorHResultOrCode    { fn eq(&self, other: &Error) -> bool { self.to_u32() == other.to_u32() } }
