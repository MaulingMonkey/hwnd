use crate::*;
use bytemuck::*;
use winapi::um::winuser::GetClientRect;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getclientrect)\]
/// GetClientRect
///
/// Retrieves the coordinates of a window's client area.
/// The client coordinates specify the upper-left and lower-right corners of the client area.
/// Because client coordinates are relative to the upper-left corner of a window's client area, the coordinates of the upper-left corner are (0,0).
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// let rect = get_client_rect(get_desktop_window()).unwrap();
/// assert_eq!(0, rect.left);
/// assert_eq!(0, rect.top);
/// assert!(0 != rect.right);
/// assert!(0 != rect.bottom);
///
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE,
///     get_client_rect(null_mut()).unwrap_err(),
/// );
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   if let Err(e) = get_client_rect((1usize << p) as HWND) { // shouldn't crash
/// #       assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e);
/// #   }
/// # }
/// ```
pub fn get_client_rect(hwnd: impl TryInto<HWnd>) -> Result<Rect, Error> {
    fn_context!(get_client_rect => GetClientRect);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let mut rect = Rect::zeroed();
    fn_succeeded!(unsafe { GetClientRect(hwnd, rect.as_mut()) })?;
    Ok(rect)
}
