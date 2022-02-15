use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_\* flags for [show_window]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct ShowWindowCmd(i32);
// TODO: .natvis

impl From<ShowWindowCmd> for i32 { fn from(cmd: ShowWindowCmd) -> Self { cmd.0 } }



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// ShowWindow
///
/// Sets `hwnd`'s show state.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is an invalid handle.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let desktop   = get_desktop_window();
/// # let vscode    = get_foreground_window();
/// #
/// # if is_zoomed(vscode) {
/// show_window(vscode, SW_MAXIMIZE).unwrap();
/// # }
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(),        SW_HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,           SW_HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(!42usize as HWND,  SW_HIDE).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW_SHOWDEFAULT     ).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW_SHOWDEFAULT     ).unwrap_err());
/// ```
///
/// ### See Also
/// *   [show_window_async]
pub fn show_window(hwnd: impl TryInto<HWND>, cmd: ShowWindowCmd) -> Result<(), Error> {
    fn_context!(show_window => ShowWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { ShowWindow(hwnd, cmd.0) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindowasync)\]
/// ShowWindowAsync
///
/// Sets `hwnd`'s show state, without waiting for the operation to complete.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is an invalid handle.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let desktop   = get_desktop_window();
/// # let vscode    = get_foreground_window();
/// #
/// # if is_zoomed(vscode) {
/// show_window_async(vscode, SW_MAXIMIZE).unwrap();
/// # }
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(),       SW_HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,          SW_HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(!42usize as HWND, SW_HIDE).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW_SHOWDEFAULT     ).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW_SHOWDEFAULT     ).unwrap_err());
/// ```
///
/// ### See Also
/// *   [show_window]
pub fn show_window_async(hwnd: impl TryInto<HWND>, cmd: ShowWindowCmd) -> Result<(), Error> {
    fn_context!(show_window_async => ShowWindowAsync);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { ShowWindowAsync(hwnd, cmd.0) })
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_HIDE
///
/// Hides the window and activates another window.
pub const SW_HIDE               : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_HIDE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNORMAL
///
/// Activates and displays a window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when displaying the window for the first time.
pub const SW_SHOWNORMAL         : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNORMAL);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_NORMAL
///
/// Activates and displays a window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when displaying the window for the first time.
pub const SW_NORMAL             : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_NORMAL);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMINIMIZED
///
/// Activates the window and displays it as a minimized window.
pub const SW_SHOWMINIMIZED      : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMINIMIZED);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMAXIMIZED
///
/// Activates the window and displays it as a maximized window.
pub const SW_SHOWMAXIMIZED      : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMAXIMIZED);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_MAXIMIZE
///
/// Activates the window and displays it as a maximized window.
pub const SW_MAXIMIZE           : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_MAXIMIZE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNOACTIVATE
///
/// Displays a window in its most recent size and position.
/// This value is similar to [SW_SHOWNORMAL], except that the window is not activated.
pub const SW_SHOWNOACTIVATE     : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNOACTIVATE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOW
///
/// Activates the window and displays it in its current size and position.
pub const SW_SHOW               : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOW);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_MINIMIZE
///
/// Minimizes the specified window and activates the next top-level window in the Z order.
pub const SW_MINIMIZE           : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_MINIMIZE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWMINNOACTIVE
///
/// Displays the window as a minimized window.
/// This value is similar to [SW_SHOWMINIMIZED], except the window is not activated.
pub const SW_SHOWMINNOACTIVE    : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWMINNOACTIVE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWNA
///
/// Displays the window in its current size and position.
/// This value is similar to [SW_SHOW], except that the window is not activated.
pub const SW_SHOWNA             : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWNA);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_RESTORE
///
/// Activates and displays the window.
/// If the window is minimized or maximized, the system restores it to its original size and position.
/// An application should specify this flag when restoring a minimized window.
pub const SW_RESTORE            : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_RESTORE);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_SHOWDEFAULT
///
/// Sets the show state based on the `SW_*` value specified in the
/// [STARTUPINFO](https://docs.microsoft.com/en-us/windows/desktop/api/processthreadsapi/ns-processthreadsapi-startupinfoa)
/// structure passed to the
/// [CreateProcess](https://docs.microsoft.com/en-us/windows/desktop/api/processthreadsapi/nf-processthreadsapi-createprocessa)
/// function by the program that started the application.
pub const SW_SHOWDEFAULT        : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_SHOWDEFAULT);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow)\]
/// SW_FORCEMINIMIZE
///
/// Minimizes a window, even if the thread that owns the window is not responding.
/// This flag should only be used when minimizing windows from a different thread.
pub const SW_FORCEMINIMIZE      : ShowWindowCmd = ShowWindowCmd(winapi::um::winuser::SW_FORCEMINIMIZE);



impl ShowWindowCmd {
    fn to_str(&self) -> &'static str {
        macro_rules! flag_to_str { ( $($ident:ident),* $(,)? ) => {
            #[allow(unreachable_patterns)]
            match *self {
                $( $ident => stringify!($ident), )*
                _ => "SW_???"
            }
        }}

        // TODO: natvis

        flag_to_str! {
            SW_HIDE,
            SW_SHOWNORMAL,
            SW_NORMAL,
            SW_SHOWMINIMIZED,
            SW_SHOWMAXIMIZED,
            SW_MAXIMIZE,
            SW_SHOWNOACTIVATE,
            SW_SHOW,
            SW_MINIMIZE,
            SW_SHOWMINNOACTIVE,
            SW_SHOWNA,
            SW_RESTORE,
            SW_SHOWDEFAULT,
        }
    }
}

impl Debug for ShowWindowCmd {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "{}", self.to_str()) }
}
