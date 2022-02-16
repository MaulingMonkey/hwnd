use crate::{*, SW::ShowWindowCmd};
use winapi::um::winuser::*;



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
/// show_window(vscode, SW::MAXIMIZE).unwrap();
/// # }
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(),        SW::HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,           SW::HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(!42usize as HWND,  SW::HIDE).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(null_mut(), SW::SHOWDEFAULT     ).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window(desktop,    SW::SHOWDEFAULT     ).unwrap_err());
/// ```
///
/// ### See Also
/// *   [show_window_async]
pub fn show_window(hwnd: impl TryInto<HWND>, cmd: ShowWindowCmd) -> Result<(), Error> {
    fn_context!(show_window => ShowWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { ShowWindow(hwnd, cmd.into()) })
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
/// show_window_async(vscode, SW::MAXIMIZE).unwrap();
/// # }
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(),       SW::HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,          SW::HIDE).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(!42usize as HWND, SW::HIDE).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(null_mut(), SW::SHOWDEFAULT     ).unwrap_err());
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWNORMAL      ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::NORMAL          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWMINIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWMAXIMIZED   ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::MAXIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWNOACTIVATE  ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOW            ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::MINIMIZE        ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWMINNOACTIVE ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWNA          ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::RESTORE         ).unwrap_err());
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, show_window_async(desktop,    SW::SHOWDEFAULT     ).unwrap_err());
/// ```
///
/// ### See Also
/// *   [show_window]
pub fn show_window_async(hwnd: impl TryInto<HWND>, cmd: ShowWindowCmd) -> Result<(), Error> {
    fn_context!(show_window_async => ShowWindowAsync);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { ShowWindowAsync(hwnd, cmd.into()) })
}
