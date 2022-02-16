use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-closewindow)\]
/// CloseWindow
///
/// Minimizes (but does not destroy) the specified window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is not a valid window handle
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # if false {
/// close_window(get_foreground_window()).unwrap(); // minimizes e.g. vscode
/// # }
/// close_window(get_desktop_window()).unwrap(); // noop
///
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE,
///     close_window(null_mut()).unwrap_err(),
/// );
///
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE,
///     close_window(!42usize as HWND).unwrap_err(),
/// );
/// ```
pub fn close_window(hwnd: impl TryInto<HWND>) -> Result<(), Error> {
    fn_context!(close_window => CloseWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { CloseWindow(hwnd) })
}
