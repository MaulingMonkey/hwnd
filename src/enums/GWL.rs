//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongw)\]
//! GWL_\* indicies for [get_window_long_w]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongw)\]
/// GWL_\* index for [get_window_long_w]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct GetWindowLongIndex(i32);

impl From<GetWindowLongIndex> for i32 { fn from(cmd: GetWindowLongIndex) -> Self { cmd.0 } }
impl From<i32> for GetWindowLongIndex { fn from(cmd: i32 ) -> Self { Self(cmd) } }

impl PartialEq<i32> for GetWindowLongIndex { fn eq(&self, other: &i32 ) -> bool { self.0 == *other } }
impl PartialEq<GetWindowLongIndex> for i32 { fn eq(&self, other: &GetWindowLongIndex) -> bool { *self == other.0 } }

impl Debug for GetWindowLongIndex {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        #![allow(deprecated)]

        macro_rules! e { ($($p:path),* $(,)?) => {
            let s = match *self {
                $($p => stringify!($p),)*
                _ => return write!(fmt, "{} (GWL::???)", self.0)
            };
            fmt.write_str(s)
        }}

        e! {
            GWL::STYLE,
            GWL::EXSTYLE,
            GWL::ID,
            GWL::WNDPROC,
            GWL::HINSTANCE,
            GWL::HWNDPARENT,
            GWL::USERDATA,
        }
    }
}



pub const STYLE         : GetWindowLongIndex = GetWindowLongIndex(GWL_STYLE);
pub const EXSTYLE       : GetWindowLongIndex = GetWindowLongIndex(GWL_EXSTYLE);
pub const ID            : GetWindowLongIndex = GetWindowLongIndex(GWL_ID);

#[cfg(not(target_pointer_width = "64"))] pub        use win64::*;
#[cfg(    target_pointer_width = "64" )] pub(crate) use win64::*;

mod win64 {
    use super::*;
    #[deprecated = "prefer GWLP::WNDPROC"      ] pub const WNDPROC       : GetWindowLongIndex = GetWindowLongIndex(GWL_WNDPROC);
    #[deprecated = "prefer GWLP::HINSTANCE"    ] pub const HINSTANCE     : GetWindowLongIndex = GetWindowLongIndex(GWL_HINSTANCE);
    #[deprecated = "prefer GWLP::HWNDPARENT"   ] pub const HWNDPARENT    : GetWindowLongIndex = GetWindowLongIndex(GWL_HWNDPARENT);
    #[deprecated = "prefer GWLP::USERDATA"     ] pub const USERDATA      : GetWindowLongIndex = GetWindowLongIndex(GWL_USERDATA);
}
