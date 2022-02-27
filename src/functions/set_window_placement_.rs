use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowplacement)\]
/// SetWindowPlacement
///
/// Retrieves the show state and the restored, minimized, and maximized positions of the specified window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::PROC_NOT_FOUND]         If `hwnd` is the desktop window, sometimes
/// *   [ERROR::INVALID_HANDLE]         If `hwnd` is the desktop window, sometimes
/// *   [ERROR::INVALID_PARAMETER]      If `wndpl` / `wndpl.length` is invalid
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
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
///
/// let e = set_window_placement(get_desktop_window(), &p);
/// assert!(
///     e == ERROR::PROC_NOT_FOUND || e == ERROR::INVALID_HANDLE || e == ERROR::INVALID_WINDOW_HANDLE,
///     "unexpected error for set_window_placement(get_desktop_window(), &p): {e:?}"
/// );
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
    fn_succeeded!(unsafe { SetWindowPlacement(hwnd, wndpl) })?;
    Ok(())
}
