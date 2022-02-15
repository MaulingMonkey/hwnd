use winapi::um::errhandlingapi::GetLastError;
use std::fmt::{self, Debug, Display, Formatter};



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Error(pub(crate) u32);

impl Error {
    pub(crate) fn new_gle() -> Self {
        Self(unsafe { GetLastError() } as _)
    }

    pub fn code(&self) -> Option<winerr::ErrorCodeMicrosoft> {
        u16::try_from(self.0).ok().map(|c| winerr::ErrorCodeMicrosoft::from(c))
    }
}

impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(fmt, "hwnd::Error({code:?})")
        } else {
            let hr = winerr::HRESULT::from(self.0);
            write!(fmt, "hwnd::Error({hr:?})")
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        if let Some(code) = self.code() {
            write!(fmt, "error handling HWNDs: {code:?}")
        } else {
            let hr = winerr::HRESULT::from(self.0);
            write!(fmt, "error handling HWNDs: {hr:?}")
        }
    }
}

impl PartialEq<winerr::ErrorCodeMicrosoft> for Error { fn eq(&self, other: &winerr::ErrorCodeMicrosoft) -> bool { self.code() == Some(*other) } }
impl PartialEq<Error> for winerr::ErrorCodeMicrosoft { fn eq(&self, other: &Error                     ) -> bool { Some(*self) == other.code() } }
