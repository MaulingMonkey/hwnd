use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)\]
/// SetWindowPlacement
///
/// Retrieves the show state and the restored, minimized, and maximized positions of the specified window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::INVALID_PARAMETER]      If `wndpl` / `wndpl.length` is invalid
/// *   [ERROR::FUNCTION_FAILED]        If the function failed without a proper error code (happens when e.g. setting the desktop hwnd placement)
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// # let mut p : WindowPlacement = get_window_placement(get_desktop_window()).unwrap();
/// # p.length = 11;
/// # #[cfg(nope)] {
/// let mut p : WindowPlacement = ..;
/// # }
///
/// p.length = std::mem::size_of_val(&p) as _;
/// set_window_placement(hwnd, &p).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_placement(HWnd::NULL,       &p), "null");
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_placement(!42usize as HWND, &p), "!42");
/// assert_eq!(ERROR::FUNCTION_FAILED,       set_window_placement(get_desktop_window(), &p));
///
/// for length in 0 .. 1000 {
///     p.length = length;
///     if p.length == std::mem::size_of_val(&p) as u32 { continue }
///     assert_eq!(ERROR::INVALID_PARAMETER, set_window_placement(hwnd, &p), "length: {length}");
/// }
/// ```
///
/// ### See Also
/// *   [WindowPlacement]
/// *   [get_window_placement]
/// *   [get_window_rect]
/// *   [set_window_pos]
pub fn set_window_placement(hwnd: impl TryInto<HWnd>, wndpl: &impl AsRef<WindowPlacement>) -> Result<(), Error> {
    fn_context!(set_window_placement => SetWindowPlacement);
    let hwnd    = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let wndpl   = wndpl.as_ref().as_ref();
    clear_last_error(); // SetWindowPlacement doesn't always set the error, even when returning FALSE for failure!
    fn_succeeded!(unsafe { SetWindowPlacement(hwnd, wndpl) }).map_err(|e| {
        if e == ERROR::SUCCESS { fn_error!(ERROR::FUNCTION_FAILED) }
        else { e }
    })
}
