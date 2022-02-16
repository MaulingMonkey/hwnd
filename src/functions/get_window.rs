use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptra)\]
/// GetWindowLongPtrA
///
/// Retrieves information about the specified window. The function also retrieves the value at a specified offset into the extra window memory.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `index` is e.g. `GWLP_WNDPROC` for a `hwnd` belonging to a different process.
/// *   [ERROR::INVALID_INDEX]          If `index` isn't valid for `hwnd`
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` isn't valid
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// let desktop_style = get_window_long_ptr_a(desktop, GWL_STYLE).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_ptr_a(null_mut(), 0).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_ptr_a(!42usize as HWND, 0).unwrap_err());
/// assert_eq!(ERROR::INVALID_INDEX,         get_window_long_ptr_a(desktop, -9001).unwrap_err());
/// assert_eq!(ERROR::ACCESS_DENIED,         get_window_long_ptr_a(desktop, GWLP_WNDPROC).unwrap_err());
/// # for i in [i32::MIN, i32::MIN/2, -9001, 0, 9001, i32::MAX/2, i32::MAX].iter().copied().chain(-64 ..= 64).chain((0..30).map(|p| 1<<p)).chain((0..31).map(|p| -(1<<p))) {
/// #   if let Err(err) = get_window_long_ptr_a(desktop, i) {
/// #       match err.code() {
/// #           Some(ERROR::ACCESS_DENIED)  => {},
/// #           Some(ERROR::INVALID_INDEX)  => {},
/// #           kind                        => panic!("get_window_long_ptr_a(desktop, {i}) == {kind:?}"),
/// #       }
/// #   }
/// # }
/// ```
pub fn get_window_long_ptr_a(hwnd: impl TryInto<HWND>, index: i32) -> Result<isize, Error> {
    fn_context!(get_window_long_ptr_a => GetWindowLongPtrA);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    let r = unsafe { GetWindowLongPtrA(hwnd, index) };
    if r == 0 { fn_error_gle_nz!()?; }
    Ok(r as _) // i32 -> isize on 32-bit windows
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowlongptrw)\]
/// GetWindowLongPtrW
///
/// Retrieves information about the specified window. The function also retrieves the value at a specified offset into the extra window memory.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `index` is e.g. `GWLP_WNDPROC` for a `hwnd` belonging to a different process.
/// *   [ERROR::INVALID_INDEX]          If `index` isn't valid for `hwnd`
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` isn't valid
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// let desktop_style = get_window_long_ptr_w(desktop, GWL_STYLE).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_ptr_w(null_mut(), 0).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_long_ptr_w(!42usize as HWND, 0).unwrap_err());
/// assert_eq!(ERROR::INVALID_INDEX,         get_window_long_ptr_w(desktop, -9001).unwrap_err());
/// assert_eq!(ERROR::ACCESS_DENIED,         get_window_long_ptr_w(desktop, GWLP_WNDPROC).unwrap_err());
/// # for i in [i32::MIN, i32::MIN/2, -9001, 0, 9001, i32::MAX/2, i32::MAX].iter().copied().chain(-64 ..= 64).chain((0..30).map(|p| 1<<p)).chain((0..31).map(|p| -(1<<p))) {
/// #   if let Err(err) = get_window_long_ptr_w(desktop, i) {
/// #       match err.code() {
/// #           Some(ERROR::ACCESS_DENIED)  => {},
/// #           Some(ERROR::INVALID_INDEX)  => {},
/// #           kind                        => panic!("get_window_long_ptr_w(desktop, {i}) == {kind:?}"),
/// #       }
/// #   }
/// # }
/// ```
pub fn get_window_long_ptr_w(hwnd: impl TryInto<HWND>, index: i32) -> Result<isize, Error> {
    fn_context!(get_window_long_ptr_w => GetWindowLongPtrW);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    let r = unsafe { GetWindowLongPtrW(hwnd, index) };
    if r == 0 { fn_error_gle_nz!()?; }
    Ok(r as _) // i32 -> isize on 32-bit windows
}
