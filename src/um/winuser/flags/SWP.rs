//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)\]
//! SWP_\* flags for [set_window_pos]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)\]
/// SWP_\* flags for [set_window_pos]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct SetWindowPosFlags(u32);
impl_ops_for_flag!(SetWindowPosFlags);

impl From<SetWindowPosFlags> for u32 { fn from(cmd: SetWindowPosFlags) -> Self { cmd.0 } }
impl From<u32> for SetWindowPosFlags { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl_debug_for_flags! {
    SetWindowPosFlags => {
        SWP::NOSIZE,
        SWP::NOMOVE,
        SWP::NOZORDER,
        SWP::NOREDRAW,
        SWP::NOACTIVATE,
        SWP::FRAMECHANGED,
        SWP::SHOWWINDOW,
        SWP::HIDEWINDOW,
        SWP::NOCOPYBITS,
        SWP::NOOWNERZORDER,
        SWP::NOSENDCHANGING,
        //SWP::DRAWFRAME,       // duplicate
        //SWP::NOREPOSITION,    // duplicate
        SWP::DEFERERASE,
        SWP::ASYNCWINDOWPOS,
    }
}



pub const NOSIZE            : SetWindowPosFlags = SetWindowPosFlags(SWP_NOSIZE);
pub const NOMOVE            : SetWindowPosFlags = SetWindowPosFlags(SWP_NOMOVE);
pub const NOZORDER          : SetWindowPosFlags = SetWindowPosFlags(SWP_NOZORDER);
pub const NOREDRAW          : SetWindowPosFlags = SetWindowPosFlags(SWP_NOREDRAW);
pub const NOACTIVATE        : SetWindowPosFlags = SetWindowPosFlags(SWP_NOACTIVATE);
pub const FRAMECHANGED      : SetWindowPosFlags = SetWindowPosFlags(SWP_FRAMECHANGED);
pub const SHOWWINDOW        : SetWindowPosFlags = SetWindowPosFlags(SWP_SHOWWINDOW);
pub const HIDEWINDOW        : SetWindowPosFlags = SetWindowPosFlags(SWP_HIDEWINDOW);
pub const NOCOPYBITS        : SetWindowPosFlags = SetWindowPosFlags(SWP_NOCOPYBITS);
pub const NOOWNERZORDER     : SetWindowPosFlags = SetWindowPosFlags(SWP_NOOWNERZORDER);
pub const NOSENDCHANGING    : SetWindowPosFlags = SetWindowPosFlags(SWP_NOSENDCHANGING);
pub const DRAWFRAME         : SetWindowPosFlags = SetWindowPosFlags(SWP_DRAWFRAME);
pub const NOREPOSITION      : SetWindowPosFlags = SetWindowPosFlags(SWP_NOREPOSITION);
pub const DEFERERASE        : SetWindowPosFlags = SetWindowPosFlags(SWP_DEFERERASE);
pub const ASYNCWINDOWPOS    : SetWindowPosFlags = SetWindowPosFlags(SWP_ASYNCWINDOWPOS);
