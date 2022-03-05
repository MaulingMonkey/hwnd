use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-destroywindow)\]
/// DestroyWindow
///
/// Destroys the specified window.
///
/// ### Safety
/// *   Destroying a window out from underneath a 3D rendering API such as Direct3D is generally unsound.
/// *   Destroying a window from it's own WM_DESTROY is unlikely to be sound.
///     While windows itself appears able to handle it, it results in multiple WM_DESTROY and WM_NCDESTROY events for the same [HWnd], and I suspect many C++ WndProcs can't tolerate that.
/// *   You should probably only destroy windows you created, and even then be careful!
/// *   See [HWnd] for more rants/details.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If `hwnd` belongs to another process
/// *   [ERROR::ACCESS_DENIED]          If `hwnd` belongs to another thread
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is not a valid window handle
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let desktop = get_desktop_window();
/// # fn create_window(_: std::ops::RangeFull) -> HWnd {
/// #   unsafe { create_window_a(
/// #       abistr::cstr!("Message"), (), 0,
/// #       0, 0, 0, 0,
/// #       HWnd::MESSAGE, null_mut(), None, null_mut()
/// #   )}.unwrap()
/// # }
/// #
/// // "Typical" usage:
/// let window = create_window(..);
/// // ...
/// unsafe { destroy_window(window) }.unwrap();
///
///
/// // Edge cases:
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE, // already destroyed
///     unsafe { destroy_window(window) }.unwrap_err(),
/// );
///
/// assert_eq!(
///     ERROR::INVALID_WINDOW_HANDLE, // invalid handle
///     unsafe { destroy_window(null_mut()) }.unwrap_err(),
/// );
///
/// assert_eq!(
///     ERROR::ACCESS_DENIED, // wrong process
///     unsafe { destroy_window(desktop) }.unwrap_err(),
/// );
///
/// let window = create_window(..);
/// assert_eq!(
///     ERROR::ACCESS_DENIED, // wrong thread
///     std::thread::spawn(move || unsafe {
///         destroy_window(window)
///     }).join().unwrap().unwrap_err(),
/// );
/// unsafe { destroy_window(window) }.unwrap();
/// ```
pub unsafe fn destroy_window(hwnd: impl TryInto<HWnd>) -> Result<(), Error> {
    fn_context!(destroy_window => DestroyWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    fn_succeeded!(unsafe { DestroyWindow(hwnd) })
}
