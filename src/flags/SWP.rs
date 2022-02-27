//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)\]
//! SWP_\* flags for [set_window_pos]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)\]
/// SWP_\* flags for [set_window_pos]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct SetWindowPosFlags(u32);
impl_ops_for_flag!(SetWindowPosFlags);

impl From<SetWindowPosFlags> for u32 { fn from(cmd: SetWindowPosFlags) -> Self { cmd.0 } }
impl From<u32> for SetWindowPosFlags { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl Debug for SetWindowPosFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "SWP::{{")?;

        let mut prev        = false;
        let mut remaining   = self.0;

        macro_rules! flags { ( $($ident:path),* $(,)? ) => {$({
            let mask : u32 = ($ident).0;
            if (remaining & mask) != 0 {
                if prev { write!(fmt, " | ")?; }
                write!(fmt, "{}", stringify!($ident))?;
                prev        = true;
                remaining   = remaining & !mask;
            }
        })*}}

        flags! {
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
            SWP::DRAWFRAME,
            SWP::NOREPOSITION,
            SWP::DEFERERASE,
            SWP::ASYNCWINDOWPOS,
        };

        let _ = (remaining, prev);

        write!(fmt, "}}")
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
