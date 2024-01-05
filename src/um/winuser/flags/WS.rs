//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)\]
//! WS_\* window style flags for [create_window_a] etc.

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/winmsg/window-styles)\]
/// WS_\* window style flags for [create_window_a] etc.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WindowStyle(u32);
impl_ops_for_flag!(WindowStyle);

impl From<WindowStyle> for u32 { fn from(cmd: WindowStyle) -> Self { cmd.0 } }
impl From<u32> for WindowStyle { fn from(cmd: u32        ) -> Self { Self(cmd) } }

impl_debug_for_flags! {
    WindowStyle => {
        // Aggregate styles
        WS::OVERLAPPEDWINDOW,   // OVERLAPPED | CAPTION | SYSMENU | THICKFRAME | MINIMIZEBOX | MAXIMIZEBOX
        WS::POPUPWINDOW,        // POPUP | BORDER | SYSMENU
        WS::CAPTION,            // BORDER | DLGFRAME

        WS::OVERLAPPED,
        WS::POPUP,
        WS::CHILD,
        WS::MINIMIZE,
        WS::VISIBLE,
        WS::DISABLED,
        WS::CLIPSIBLINGS,
        WS::CLIPCHILDREN,
        WS::MAXIMIZE,
        WS::BORDER,
        WS::DLGFRAME,
        WS::VSCROLL,
        WS::HSCROLL,
        WS::SYSMENU,
        WS::THICKFRAME,
        //WS::GROUP,        // overlaps with WS::MINIMIZEBOX
        //WS::TABSTOP,      // overlaps with WS::MAXIMIZEBOX
        WS::MINIMIZEBOX,
        WS::MAXIMIZEBOX,
        //WS::TILED,        // alias for WS::OVERLAPPED
        //WS::ICONIC,       // alias for WS::MINIMIZE
        //WS::SIZEBOX,      // alias for WS::THICKFRAME
        //WS::TILEDWINDOW,  // alias for WS::OVERLAPPEDWINDOW
        //WS::CHILDWINDOW,  // alias for WS::CHILD
    }
}



pub const OVERLAPPED        : WindowStyle = WindowStyle(WS_OVERLAPPED);
pub const POPUP             : WindowStyle = WindowStyle(WS_POPUP);
pub const CHILD             : WindowStyle = WindowStyle(WS_CHILD);
pub const MINIMIZE          : WindowStyle = WindowStyle(WS_MINIMIZE);
pub const VISIBLE           : WindowStyle = WindowStyle(WS_VISIBLE);
pub const DISABLED          : WindowStyle = WindowStyle(WS_DISABLED);
pub const CLIPSIBLINGS      : WindowStyle = WindowStyle(WS_CLIPSIBLINGS);
pub const CLIPCHILDREN      : WindowStyle = WindowStyle(WS_CLIPCHILDREN);
pub const MAXIMIZE          : WindowStyle = WindowStyle(WS_MAXIMIZE);
pub const CAPTION           : WindowStyle = WindowStyle(WS_CAPTION);
pub const BORDER            : WindowStyle = WindowStyle(WS_BORDER);
pub const DLGFRAME          : WindowStyle = WindowStyle(WS_DLGFRAME);
pub const VSCROLL           : WindowStyle = WindowStyle(WS_VSCROLL);
pub const HSCROLL           : WindowStyle = WindowStyle(WS_HSCROLL);
pub const SYSMENU           : WindowStyle = WindowStyle(WS_SYSMENU);
pub const THICKFRAME        : WindowStyle = WindowStyle(WS_THICKFRAME);
pub const GROUP             : WindowStyle = WindowStyle(WS_GROUP);
pub const TABSTOP           : WindowStyle = WindowStyle(WS_TABSTOP);
pub const MINIMIZEBOX       : WindowStyle = WindowStyle(WS_MINIMIZEBOX);
pub const MAXIMIZEBOX       : WindowStyle = WindowStyle(WS_MAXIMIZEBOX);
pub const TILED             : WindowStyle = WindowStyle(WS_TILED);
pub const ICONIC            : WindowStyle = WindowStyle(WS_ICONIC);
pub const SIZEBOX           : WindowStyle = WindowStyle(WS_SIZEBOX);
pub const TILEDWINDOW       : WindowStyle = WindowStyle(WS_TILEDWINDOW);
pub const OVERLAPPEDWINDOW  : WindowStyle = WindowStyle(WS_OVERLAPPEDWINDOW);
pub const POPUPWINDOW       : WindowStyle = WindowStyle(WS_POPUPWINDOW);
pub const CHILDWINDOW       : WindowStyle = WindowStyle(WS_CHILDWINDOW);
