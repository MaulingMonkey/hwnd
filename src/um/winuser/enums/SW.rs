//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
//! SW_\* commands for [show_window]\[[_async](show_window_async)\]
#![allow(non_snake_case)]
#![allow(unreachable_patterns)]

use crate::*;
use bytemuck::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_\* command for [show_window]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct ShowWindowCmd(i32);
// TODO: .natvis

impl From<ShowWindowCmd> for i32 { fn from(cmd: ShowWindowCmd) -> Self { cmd.0 } }

impl_debug_for_enum! {
    ShowWindowCmd => {
        SW::HIDE,
        SW::SHOWNORMAL,
        SW::NORMAL,
        SW::SHOWMINIMIZED,
        SW::SHOWMAXIMIZED,
        SW::MAXIMIZE,
        SW::SHOWNOACTIVATE,
        SW::SHOW,
        SW::MINIMIZE,
        SW::SHOWMINNOACTIVE,
        SW::SHOWNA,
        SW::RESTORE,
        SW::SHOWDEFAULT,
    }
}



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_HIDE
///
/// Hides the window and activates another window.
pub const HIDE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_HIDE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNORMAL
///
/// Activates and displays a window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when displaying the window for the first time.
pub const SHOWNORMAL : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNORMAL);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_NORMAL
///
/// Activates and displays a window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when displaying the window for the first time.
pub const NORMAL : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_NORMAL);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMINIMIZED
///
/// Activates the window and displays it as a minimized window.
pub const SHOWMINIMIZED : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMINIMIZED);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMAXIMIZED
///
/// Activates the window and displays it as a maximized window.
pub const SHOWMAXIMIZED : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMAXIMIZED);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_MAXIMIZE
///
/// Activates the window and displays it as a maximized window.
pub const MAXIMIZE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_MAXIMIZE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNOACTIVATE
///
/// Displays a window in its most recent size and position.
/// This value is similar to [SW::SHOWNORMAL], except that the window is not activated.
pub const SHOWNOACTIVATE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNOACTIVATE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOW
///
/// Activates the window and displays it in its current size and position.
pub const SHOW : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOW);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_MINIMIZE
///
/// Minimizes the specified window and activates the next top-level window in the Z order.
pub const MINIMIZE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_MINIMIZE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMINNOACTIVE
///
/// Displays the window as a minimized window.
/// This value is similar to [SW::SHOWMINIMIZED], except the window is not activated.
pub const SHOWMINNOACTIVE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMINNOACTIVE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNA
///
/// Displays the window in its current size and position.
/// This value is similar to [SW::SHOW], except that the window is not activated.
pub const SHOWNA : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNA);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_RESTORE
///
/// Activates and displays the window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when restoring a minimized window.
pub const RESTORE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_RESTORE);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWDEFAULT
///
/// Sets the show state based on the `SW_*` value specified in the
/// [STARTUPINFO](https://learn.microsoft.com/en-us/windows/desktop/api/processthreadsapi/ns-processthreadsapi-startupinfoa)
/// structure passed to the
/// [CreateProcess](https://learn.microsoft.com/en-us/windows/desktop/api/processthreadsapi/nf-processthreadsapi-createprocessa)
/// function by the program that started the application.
pub const SHOWDEFAULT : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWDEFAULT);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_FORCEMINIMIZE
///
/// Minimizes a window, even if the thread that owns the window is not responding.
/// This flag should only be used when minimizing windows from a different thread.
pub const FORCEMINIMIZE : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_FORCEMINIMIZE);
