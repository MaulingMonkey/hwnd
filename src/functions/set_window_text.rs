use crate::*;
use abistr::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtexta)\]
/// SetWindowTextA
///
/// Changes the text of the specified window's title bar (if it has one).
/// If the specified window is a control, the text of the control is changed.
/// However, [set_window_text_a] cannot change the text of a control in another application.
/// (It will silently fail.)
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::INVALID_PARAMETER]      If `string` contains interior `\0`s
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use abistr::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_a(0, abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// set_window_text_a(hwnd, abistr::cstr!("text")   ).unwrap();
/// set_window_text_a(hwnd, ()                      ).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_a(HWnd::BROADCAST,         abistr::cstr!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_a(HWnd::MESSAGE,           abistr::cstr!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_a(!42usize as HWND,        abistr::cstr!("text")));
/// assert_eq!(Ok(()),                       set_window_text_a(get_desktop_window(),    abistr::cstr!("text")));
/// ```
///
/// ### See Also
/// *   [set_window_text_w]
pub fn set_window_text_a(hwnd: impl TryInto<HWnd>, string: impl TryIntoAsOptCStr) -> Result<(), Error> {
    fn_context!(set_window_text_a => SetWindowTextA);
    let hwnd    = hwnd  .try_into().map_err(|_| fn_param_error!(hwnd,   ERROR::INVALID_WINDOW_HANDLE))?.into();
    let string  = string.try_into().map_err(|_| fn_param_error!(string, ERROR::INVALID_PARAMETER))?;
    fn_succeeded!(unsafe { SetWindowTextA(hwnd, string.as_opt_cstr()) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)\]
/// SetWindowTextW
///
/// Changes the text of the specified window's title bar (if it has one).
/// If the specified window is a control, the text of the control is changed.
/// However, [set_window_text_w] cannot change the text of a control in another application.
/// (It will silently fail.)
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::INVALID_PARAMETER]      If `string` contains interior `\0`s
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use abistr::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// set_window_text_w(hwnd, abistr::cstr16!("text") ).unwrap();
/// set_window_text_w(hwnd, ()                      ).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(HWnd::BROADCAST,         abistr::cstr16!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(HWnd::MESSAGE,           abistr::cstr16!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(!42usize as HWND,        abistr::cstr16!("text")));
/// assert_eq!(Ok(()),                       set_window_text_w(get_desktop_window(),    abistr::cstr16!("text")));
/// ```
///
/// ### See Also
/// *   [set_window_text_a]
pub fn set_window_text_w(hwnd: impl TryInto<HWnd>, string: impl TryIntoAsOptCStr<u16>) -> Result<(), Error> {
    fn_context!(set_window_text_w => SetWindowTextW);
    let hwnd    = hwnd  .try_into().map_err(|_| fn_param_error!(hwnd,   ERROR::INVALID_WINDOW_HANDLE))?.into();
    let string  = string.try_into().map_err(|_| fn_param_error!(string, ERROR::INVALID_PARAMETER))?;
    fn_succeeded!(unsafe { SetWindowTextW(hwnd, string.as_opt_cstr()) })
}
