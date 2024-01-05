use crate::*;
use winapi::um::winuser::*;
use bytemuck::Zeroable;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowplacement)\]
/// GetWindowPlacement
///
/// Retrieves the show state and the restored, minimized, and maximized positions of the specified window.
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// # let p = get_window_placement(get_desktop_window()).unwrap();
/// let p = get_window_placement(hwnd).unwrap();
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_placement(HWnd::NULL));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_placement(!42usize as HWND));
/// ```
///
/// ### See Also
/// *   [WindowPlacement]
/// *   [get_window_rect]
/// *   [set_window_placement]
/// *   [set_window_pos]
pub fn get_window_placement(hwnd: impl TryInto<HWnd>) -> Result<WindowPlacement, Error> {
    fn_context!(get_window_placement => GetWindowPlacement);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    let mut wndpl = WindowPlacement::zeroed();
    wndpl.length = size_of_32::<WindowPlacement>();
    fn_succeeded!(unsafe { GetWindowPlacement(hwnd, wndpl.as_mut()) })?;
    Ok(wndpl)
}
