use crate::*;
use crate::SWP::SetWindowPosFlags;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setwindowpos)\]
/// SetWindowPos
///
/// Changes the size, position, and Z order of a child, pop-up, or top-level window.
/// These windows are ordered according to their appearance on the screen.
/// The topmost window receives the highest rank and is the first window in the Z order.
///
/// ### Arguments
/// *   `hwnd`  should be a valid non-null window handle.
/// *   `hwnd_insert_after` should be a valid window handle, or one of:
///     *   [HWnd::BOTTOM]
///     *   [HWnd::NOTOPMOST]
///     *   [HWnd::TOP]
///     *   [HWnd::TOPMOST]
/// *   `hwnd_insert_after` shouldn't be:
///     *   [HWnd::NULL]        (treated as [HWnd::TOP])
///     *   [HWnd::BROADCAST]   (results in [ERROR::INVALID_WINDOW_HANDLE])
///     *   [HWnd::MESSAGE]     (results in [ERROR::INVALID_WINDOW_HANDLE])
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` or `hwnd_insert_after` is invalid
/// *   [ERROR::PROC_NOT_FOUND]         Sometimes, if a wndproc could not be found? (Seems to happen "randomly" if `hwnd` is the desktop window?)
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_ex_w(0, abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
/// set_window_pos(hwnd, HWnd::TOP, 0, 0, 0, 0, SWP::NOMOVE | SWP::NOSIZE).unwrap();
/// set_window_pos(hwnd, HWnd::TOP, -10000, -10000, -10000, -10000, 0).unwrap();
///
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_pos(hwnd, HWnd::BROADCAST,  0, 0, 0, 0, SWP::NOMOVE | SWP::NOSIZE));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_pos(hwnd, HWnd::MESSAGE,    0, 0, 0, 0, SWP::NOMOVE | SWP::NOSIZE));
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, set_window_pos(hwnd, !42usize as HWND, 0, 0, 0, 0, SWP::NOMOVE | SWP::NOSIZE));
///
/// let e = set_window_pos(get_desktop_window(), HWnd::TOP, 0, 0, 0, 0, SWP::NOMOVE | SWP::NOSIZE);
/// assert!(e == ERROR::INVALID_WINDOW_HANDLE || e == ERROR::PROC_NOT_FOUND, "{e:?}"); // inconsistent error?
/// ```
pub fn set_window_pos(hwnd: impl TryInto<HWnd>, hwnd_insert_after: impl TryInto<HWnd>, x: i32, y: i32, width: i32, height: i32, flags: impl Into<SetWindowPosFlags>) -> Result<(), Error> {
    fn_context!(set_window_pos => SetWindowPos);
    let hwnd                = hwnd              .try_into().map_err(|_| fn_param_error!(hwnd,               ERROR::INVALID_WINDOW_HANDLE))?.into();
    let hwnd_insert_after   = hwnd_insert_after .try_into().map_err(|_| fn_param_error!(hwnd_insert_after,  ERROR::INVALID_WINDOW_HANDLE))?.into();
    let flags               = flags.into().into();
    fn_succeeded!(unsafe { SetWindowPos(hwnd, hwnd_insert_after, x, y, width, height, flags) })
}
