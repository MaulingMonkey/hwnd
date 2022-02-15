use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)\]
/// DestroyWindow
///
/// Destroys the specified window.
///
/// ### Safety
/// *   Destroying a window out from underneath a 3D rendering API such as Direct3D is generally unsound.
/// *   Destroying a "destroyed" window is unlikely to be sound.  While windows itself can handle it,
///     it can result in multiple WM_DESTROY and WM_NCDESTROY events, which the underlying wndprocs likely can't.
/// *   Destroying a window belonging to another thread or process is an incredibly bad idea, if it even works.
/// *   Honestly, you should probably only destroy windows you created, in your own process, on your own thread, and even then be careful!
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `hwnd` belongs to another process
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is not a valid window handle
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE,
///     unsafe { destroy_window(null_mut()) }.err().unwrap().code().unwrap(),
/// );
///
/// assert_eq!(
///     ERROR::ACCESS_DENIED,
///     unsafe { destroy_window(desktop) }.err().unwrap().code().unwrap(),
/// );
/// ```
pub unsafe fn destroy_window(hwnd: impl TryInto<HWND>) -> Result<(), Error> {
    fn_context!(destroy_window => DestroyWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?;
    fn_succeeded!(unsafe { DestroyWindow(hwnd) })
}
