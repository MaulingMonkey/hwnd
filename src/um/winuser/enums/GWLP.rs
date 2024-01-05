//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
//! GWLP_\* indicies for [get_window_long_ptr_w]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
/// GWLP_\* index for [get_window_long_ptr_w]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct GetWindowLongPtrIndex(i32);

impl From<GetWindowLongPtrIndex> for i32 { fn from(cmd: GetWindowLongPtrIndex) -> Self { cmd.0 } }
impl From<i32> for GetWindowLongPtrIndex { fn from(cmd: i32 ) -> Self { Self(cmd) } }

impl PartialEq<i32> for GetWindowLongPtrIndex { fn eq(&self, other: &i32 ) -> bool { self.0 == *other } }
impl PartialEq<GetWindowLongPtrIndex> for i32 { fn eq(&self, other: &GetWindowLongPtrIndex) -> bool { *self == other.0 } }

impl_debug_for_enum! {
    GetWindowLongPtrIndex => {
        GWLP::WNDPROC,
        GWLP::HINSTANCE,
        GWLP::HWNDPARENT,
        GWLP::USERDATA,
        GWLP::ID,
    }
}



pub const WNDPROC      : GetWindowLongPtrIndex = GetWindowLongPtrIndex(GWLP_WNDPROC);
pub const HINSTANCE    : GetWindowLongPtrIndex = GetWindowLongPtrIndex(GWLP_HINSTANCE);
pub const HWNDPARENT   : GetWindowLongPtrIndex = GetWindowLongPtrIndex(GWLP_HWNDPARENT);
pub const USERDATA     : GetWindowLongPtrIndex = GetWindowLongPtrIndex(GWLP_USERDATA);
pub const ID           : GetWindowLongPtrIndex = GetWindowLongPtrIndex(GWLP_ID);
