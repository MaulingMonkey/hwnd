use crate::*;
use abistr::{*, encoding::Utf16ish};
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowtextw)\]
/// SetWindowTextW
///
/// Changes the text of the specified window's title bar (if it has one).
/// If the specified window is a control, the text of the control is changed.
/// The docs claim [set_window_text_w] cannot change the text of a control in another application.
/// However, testing reveals it works fine on at least the desktop window.
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
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// set_window_text_w(hwnd, abistr::utf16ish!("text") ).unwrap();
/// set_window_text_w(hwnd, ()                      ).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(HWnd::BROADCAST,         abistr::utf16ish!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(HWnd::MESSAGE,           abistr::utf16ish!("text")));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_text_w(!42usize as HWND,        abistr::utf16ish!("text")));
/// # if false { // just because we can, doesn't mean we should (mucks with get_window_text unit tests later!)
/// assert_eq!(Ok(()),                       set_window_text_w(get_desktop_window(),    abistr::utf16ish!("text")));
/// # }
/// ```
pub fn set_window_text_w(hwnd: impl TryInto<HWnd>, string: impl TryIntoAsOptCStr<Utf16ish>) -> Result<(), Error> {
    fn_context!(set_window_text_w => SetWindowTextW);
    let hwnd    = hwnd  .try_into().map_err(|_| fn_param_error!(hwnd,   ERROR::INVALID_WINDOW_HANDLE))?.into();
    let string  = string.try_into().map_err(|_| fn_param_error!(string, ERROR::INVALID_PARAMETER))?;
    fn_succeeded!(unsafe { SetWindowTextW(hwnd, string.as_opt_cstr()) })
}
