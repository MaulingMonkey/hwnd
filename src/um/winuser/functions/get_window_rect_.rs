use crate::*;
use bytemuck::Zeroable;
use winapi::um::winuser::GetWindowRect;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowrect)\]
/// GetWindowRect
///
/// Retrieves the dimensions of the bounding rectangle of the specified window.
/// The dimensions are given in screen coordinates that are relative to the upper-left corner of the screen.
///
/// GetWindowRect is virtualized for DPI.
///
/// In Windows Vista and later, this includes the area occupied by the drop shadow.
///
/// To get the window bounds excluding the drop shadow, use
/// [DwmGetWindowAttribute](https://docs.microsoft.com/en-us/windows/win32/api/dwmapi/nf-dwmapi-dwmgetwindowattribute),
/// specifying DWMWA_EXTENDED_FRAME_BOUNDS.
/// Note that unlike the Window Rect, the DWM Extended Frame Bounds are not adjusted for DPI.
/// Getting the extended frame bounds can only be done after the window has been shown at least once.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// # let hwnd = unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// #
/// let rect : Rect = get_window_rect(hwnd                ).unwrap();
/// let rect : Rect = get_window_rect(get_desktop_window()).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_rect(null_mut()      ).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_rect(HWnd::BROADCAST ).unwrap_err());
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_rect(!42usize as HWND).unwrap_err());
/// ```
///
/// ### See Also
/// *   [get_client_rect]
/// *   [set_window_pos]
pub fn get_window_rect(hwnd: impl TryInto<HWnd>) -> Result<Rect, Error> {
    fn_context!(get_window_rect => GetWindowRect);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let mut rect = Rect::zeroed();
    fn_succeeded!(unsafe { GetWindowRect(hwnd, rect.as_mut()) })?;
    Ok(rect)
}
