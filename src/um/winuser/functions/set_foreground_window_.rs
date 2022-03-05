use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-allowsetforegroundwindow)\]
/// ASFW_ANY
///
/// Used with [allow_set_foreground_window] to allow any process to set the foreground window.
pub use winapi::um::winuser::ASFW_ANY;

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-allowsetforegroundwindow)\]
/// AllowSetForegroundWindow
///
/// **NOTE:** There are many restrictions on when a process can set (or grant the ability to set) the foreground window.  See [set_foreground_window] for details.
///
/// Enables the specified process to set the foreground window using the [set_foreground_window] function.
/// The calling process must already be able to set the foreground window.
///
/// ### Arguments
/// *   `process_id`    The process ID to give access to [set_foreground_window], or [`ASFW_ANY`] to allow any process.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]      If the current process is itself restricted from setting the foreground window.  See [set_foreground_window] for details.
/// *   [ERROR::INVALID_PARAMETER]  If `process_id` isn't a valid process ID
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # let pid = ASFW_ANY;
/// let _ = allow_set_foreground_window(pid); // typical use
///
/// if let Err(e) = allow_set_foreground_window(ASFW_ANY) {
///     match e.code() {
///         None                        => {}, // typical
///         Some(ERROR::ACCESS_DENIED)  => {}, // semi-common (focus another window and type while launching tests)
///         _ => panic!("allow_set_foreground_window(ASFW_ANY): unexpected error: {e:?}"),
///     }
/// }
///
/// if let Err(e) = allow_set_foreground_window(!42) {
///     match e.code() {
///         Some(ERROR::INVALID_PARAMETER)  => {}, // typical
///         Some(ERROR::ACCESS_DENIED)      => {}, // semi-common (focus another window and type while launching tests)
///         None                            => {}, // unlikely, but maybe pid !42 existed?
///         _ => panic!("allow_set_foreground_window(!42): unexpected error: {e:?}"),
///     }
/// }
/// ```
pub fn allow_set_foreground_window(process_id: u32) -> Result<(), Error> {
    fn_context!(allow_set_foreground_window => AllowSetForegroundWindow);
    fn_succeeded!(unsafe { AllowSetForegroundWindow(process_id) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setforegroundwindow)\]
/// SetForegroundWindow
///
/// **NOTE:** There are many restrictions on when a process can set the foreground window.
///
/// Brings the thread that created the specified window into the foreground and activates the window.
/// Keyboard input is directed to the window, and various visual cues are changed for the user.
/// The system assigns a slightly higher priority to the thread that created the foreground window than it does to other threads.
///
/// ### Errors
/// *   [ERROR::ACCESS_DENIED]          If restricted from setting the foreground window (see "Restrictions" bellow.)
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If restricted from setting the foreground window, if given `desktop` (or other [HWnd]s belonging to other processes?)
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd`'s try_into fails, or results in a null or otherwise invalid window handle.
///
/// ### Examples
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::null_mut;
/// # let desktop   = get_desktop_window();
/// # let hwnd      = get_foreground_window();
/// # if !hwnd.is_null() {
/// let _ = set_foreground_window(hwnd); // typical use
/// # }
///
/// let e = set_foreground_window(null_mut()).err().unwrap();
/// match e.code() {
///     Some(ERROR::INVALID_WINDOW_HANDLE)  => {}, // typical
///     Some(ERROR::ACCESS_DENIED)          => {}, // semi-common (focus another window and type while launching tests)
///     _ => panic!("set_foreground_window(null_mut()): unexpected error: {e:?}"),
/// }
///
/// if let Err(e) = set_foreground_window(!42usize as HWND) {
///     match e.code() {
///         Some(ERROR::INVALID_WINDOW_HANDLE)  => {}, // typical
///         _ => panic!("set_foreground_window(!42): unexpected error: {e:?}"),
///     }
/// }
///
/// if let Err(e) = set_foreground_window(desktop) {
///     match e.code() {
///         Some(ERROR::ACCESS_DENIED)          => {}, // expected when tests unfocused, but instead we get:
///         Some(ERROR::INVALID_WINDOW_HANDLE)  => {}, // ... on Windows 10.0.19041.1415
///         _ => panic!("set_foreground_window(desktop): unexpected error: {e:?}"),
///     }
/// }
// TODO: create a local window
/// ```
///
/// ### Restrictions
/// A process can set the foreground window only if one of the following conditions is true:
/// *   The process is the foreground process.
/// *   The process was started by the foreground process.
/// *   The process received the last input event.
/// *   There is no foreground process.
/// *   The process is being debugged.
/// *   The foreground process is not a Modern Application or the Start Screen.
/// *   The foreground is not locked (see LockSetForegroundWindow).
/// *   The foreground lock time-out has expired (see SPI_GETFOREGROUNDLOCKTIMEOUT in SystemParametersInfo).
/// *   No menus are active.
pub fn set_foreground_window(hwnd: impl TryInto<HWnd>) -> Result<(), Error> {
    fn_context!(set_foreground_window => SetForegroundWindow);
    let hwnd = hwnd.try_into().map_err(|_| fn_param_error!(hwnd, ERROR::INVALID_WINDOW_HANDLE))?.into();
    fn_succeeded!(unsafe { SetForegroundWindow(hwnd) })
}
