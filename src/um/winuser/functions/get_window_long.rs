use crate::*;
use crate::GWL::GetWindowLongIndex;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongw)\]
/// GetWindowLongW
///
/// Retrieves information about the specified window. The function also retrieves the value at a specified offset into the extra window memory.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `index` is e.g. [GWLP::WNDPROC] for a `hwnd` belonging to a different process.
/// *   [ERROR::INVALID_INDEX]          If `index` is e.g. [GWLP::WNDPROC] for a `hwnd` belonging to 64-bit process.
/// *   [ERROR::INVALID_INDEX]          If `index` isn't valid for `hwnd`
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` isn't valid
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// let desktop_style = get_window_long_w(desktop, GWL::STYLE).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_w(null_mut(), 0));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_w(!42usize as HWND, 0));
/// assert_eq!(ERROR::INVALID_INDEX,         get_window_long_w(desktop, -9001));
/// assert_eq!(ERROR::INVALID_INDEX,         get_window_long_w(desktop, i32::from(GWLP::WNDPROC)));
/// # for i in [i32::MIN, i32::MIN/2, -9001, 0, 9001, i32::MAX/2, i32::MAX].iter().copied().chain(-64 ..= 64).chain((0..30).map(|p| 1<<p)).chain((0..31).map(|p| -(1<<p))) {
/// #   if let Err(err) = get_window_long_w(desktop, i) {
/// #       match err.code() {
/// #           Some(ERROR::ACCESS_DENIED)  => {},
/// #           Some(ERROR::INVALID_INDEX)  => {},
/// #           kind                        => panic!("get_window_long_w(desktop, {i}) == {kind:?}"),
/// #       }
/// #   }
/// # }
/// ```
///
/// ### See Also
/// *   [GWL]
/// *   [get_window_long_ptr_w]
pub fn get_window_long_w(hwnd: impl TryInto<HWnd>, index: impl Into<GetWindowLongIndex>) -> Result<isize, Error> {
    fn_context!(get_window_long_w => GetWindowLongW);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let index = index.into().into();
    clear_last_error(); // GetWindowLongW might return 0 without clearing the error
    let r = unsafe { GetWindowLongW(hwnd, index) };
    if r == 0 { fn_error_gle_nz!()?; }
    Ok(r as _) // i32 -> isize on 32-bit windows
}
