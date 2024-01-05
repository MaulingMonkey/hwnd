//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)\]
//! WPF_\* flags for [WindowPlacement]

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-windowplacement)\]
/// WPF_\* flags for [WindowPlacement]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WindowPlacementFlags(u32);
impl_ops_for_flag!(WindowPlacementFlags);

impl From<WindowPlacementFlags> for u32 { fn from(cmd: WindowPlacementFlags) -> Self { cmd.0 } }
impl From<u32> for WindowPlacementFlags { fn from(cmd: u32                 ) -> Self { Self(cmd) } }

impl_debug_for_flags! {
    WindowPlacementFlags => {
        WPF::SETMINPOSITION,
        WPF::RESTORETOMAXIMIZED,
        WPF::ASYNCWINDOWPLACEMENT,
    }
}



pub const SETMINPOSITION        : WindowPlacementFlags = WindowPlacementFlags(WPF_SETMINPOSITION);
pub const RESTORETOMAXIMIZED    : WindowPlacementFlags = WindowPlacementFlags(WPF_RESTORETOMAXIMIZED);
pub const ASYNCWINDOWPLACEMENT  : WindowPlacementFlags = WindowPlacementFlags(WPF_ASYNCWINDOWPLACEMENT);
