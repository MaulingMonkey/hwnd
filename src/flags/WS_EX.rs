//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)\]
//! WS_EX_\* extended window style flags for [create_window_ex_a] etc.

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/winmsg/extended-window-styles)\]
/// WS_EX_\* extended window style flags for [create_window_ex_a] etc.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct WindowStyleExtended(u32);
impl_ops_for_flag!(WindowStyleExtended);

impl From<WindowStyleExtended> for u32 { fn from(cmd: WindowStyleExtended) -> Self { cmd.0 } }
impl From<u32> for WindowStyleExtended { fn from(cmd: u32                ) -> Self { Self(cmd) } }

impl Debug for WindowStyleExtended {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "WS_EX::{{")?;

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
            WS_EX::DLGMODALFRAME,
            WS_EX::NOPARENTNOTIFY,
            WS_EX::TOPMOST,
            WS_EX::ACCEPTFILES,
            WS_EX::TRANSPARENT,
            WS_EX::MDICHILD,
            WS_EX::TOOLWINDOW,
            WS_EX::WINDOWEDGE,
            WS_EX::CLIENTEDGE,
            WS_EX::CONTEXTHELP,
            WS_EX::RIGHT,
            WS_EX::LEFT,
            WS_EX::RTLREADING,
            WS_EX::LTRREADING,
            WS_EX::LEFTSCROLLBAR,
            WS_EX::RIGHTSCROLLBAR,
            WS_EX::CONTROLPARENT,
            WS_EX::STATICEDGE,
            WS_EX::APPWINDOW,
            WS_EX::OVERLAPPEDWINDOW,
            WS_EX::PALETTEWINDOW,
            WS_EX::LAYERED,
            WS_EX::NOINHERITLAYOUT,
            WS_EX::NOREDIRECTIONBITMAP,
            WS_EX::LAYOUTRTL,
            WS_EX::COMPOSITED,
            WS_EX::NOACTIVATE,
        };

        let _ = (remaining, prev);

        write!(fmt, "}}")
    }
}



pub const DLGMODALFRAME       : WindowStyleExtended = WindowStyleExtended(WS_EX_DLGMODALFRAME);
pub const NOPARENTNOTIFY      : WindowStyleExtended = WindowStyleExtended(WS_EX_NOPARENTNOTIFY);
pub const TOPMOST             : WindowStyleExtended = WindowStyleExtended(WS_EX_TOPMOST);
pub const ACCEPTFILES         : WindowStyleExtended = WindowStyleExtended(WS_EX_ACCEPTFILES);
pub const TRANSPARENT         : WindowStyleExtended = WindowStyleExtended(WS_EX_TRANSPARENT);
pub const MDICHILD            : WindowStyleExtended = WindowStyleExtended(WS_EX_MDICHILD);
pub const TOOLWINDOW          : WindowStyleExtended = WindowStyleExtended(WS_EX_TOOLWINDOW);
pub const WINDOWEDGE          : WindowStyleExtended = WindowStyleExtended(WS_EX_WINDOWEDGE);
pub const CLIENTEDGE          : WindowStyleExtended = WindowStyleExtended(WS_EX_CLIENTEDGE);
pub const CONTEXTHELP         : WindowStyleExtended = WindowStyleExtended(WS_EX_CONTEXTHELP);
pub const RIGHT               : WindowStyleExtended = WindowStyleExtended(WS_EX_RIGHT);
pub const LEFT                : WindowStyleExtended = WindowStyleExtended(WS_EX_LEFT);
pub const RTLREADING          : WindowStyleExtended = WindowStyleExtended(WS_EX_RTLREADING);
pub const LTRREADING          : WindowStyleExtended = WindowStyleExtended(WS_EX_LTRREADING);
pub const LEFTSCROLLBAR       : WindowStyleExtended = WindowStyleExtended(WS_EX_LEFTSCROLLBAR);
pub const RIGHTSCROLLBAR      : WindowStyleExtended = WindowStyleExtended(WS_EX_RIGHTSCROLLBAR);
pub const CONTROLPARENT       : WindowStyleExtended = WindowStyleExtended(WS_EX_CONTROLPARENT);
pub const STATICEDGE          : WindowStyleExtended = WindowStyleExtended(WS_EX_STATICEDGE);
pub const APPWINDOW           : WindowStyleExtended = WindowStyleExtended(WS_EX_APPWINDOW);
pub const OVERLAPPEDWINDOW    : WindowStyleExtended = WindowStyleExtended(WS_EX_OVERLAPPEDWINDOW);
pub const PALETTEWINDOW       : WindowStyleExtended = WindowStyleExtended(WS_EX_PALETTEWINDOW);
pub const LAYERED             : WindowStyleExtended = WindowStyleExtended(WS_EX_LAYERED);
pub const NOINHERITLAYOUT     : WindowStyleExtended = WindowStyleExtended(WS_EX_NOINHERITLAYOUT);
pub const NOREDIRECTIONBITMAP : WindowStyleExtended = WindowStyleExtended(WS_EX_NOREDIRECTIONBITMAP);
pub const LAYOUTRTL           : WindowStyleExtended = WindowStyleExtended(WS_EX_LAYOUTRTL);
pub const COMPOSITED          : WindowStyleExtended = WindowStyleExtended(WS_EX_COMPOSITED);
pub const NOACTIVATE          : WindowStyleExtended = WindowStyleExtended(WS_EX_NOACTIVATE);

