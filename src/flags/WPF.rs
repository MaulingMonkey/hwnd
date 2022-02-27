//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)\]
//! WPF_\* flags for [WindowPlacement]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)\]
/// WPF_\* flags for [WindowPlacement]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WindowPlacementFlags(u32);
impl_ops_for_flag!(WindowPlacementFlags);

impl From<WindowPlacementFlags> for u32 { fn from(cmd: WindowPlacementFlags) -> Self { cmd.0 } }
impl From<u32> for WindowPlacementFlags { fn from(cmd: u32                 ) -> Self { Self(cmd) } }

impl Debug for WindowPlacementFlags {
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
            WPF::SETMINPOSITION,
            WPF::RESTORETOMAXIMIZED,
            WPF::ASYNCWINDOWPLACEMENT,
        };

        let _ = (remaining, prev);

        write!(fmt, "}}")
    }
}



pub const SETMINPOSITION        : WindowPlacementFlags = WindowPlacementFlags(WPF_SETMINPOSITION);
pub const RESTORETOMAXIMIZED    : WindowPlacementFlags = WindowPlacementFlags(WPF_RESTORETOMAXIMIZED);
pub const ASYNCWINDOWPLACEMENT  : WindowPlacementFlags = WindowPlacementFlags(WPF_ASYNCWINDOWPLACEMENT);
