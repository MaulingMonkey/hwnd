use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getactivewindow)\]
/// GetActiveWindow
///
/// Retrieves the window handle to the active window attached to the calling thread's message queue.
///
/// ### Returns
/// *   The active window attached to the calling thread's message queue.
/// *   Otherwise, NULL.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let active : HWND = get_active_window();
/// assert!(active.is_null(), "unit tests don't have an active window");
/// ```
pub fn get_active_window() -> HWND {
    fn_context!(get_active_window => GetActiveWindow);
    unsafe { GetActiveWindow() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)\]
/// GetDesktopWindow
///
/// Retrieves a handle to the desktop window.
/// The desktop window covers the entire screen.
/// The desktop window is the area on top of which other windows are painted.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let desktop : HWND = get_desktop_window();
/// assert!(!desktop.is_null(), "the desktop should never be null");
/// ```
pub fn get_desktop_window() -> HWND {
    fn_context!(get_desktop_window => GetDesktopWindow);
    unsafe { GetDesktopWindow() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getdesktopwindow)\]
/// GetForegroundWindow
///
/// Retrieves a handle to the foreground window (the window with which the user is currently working).
/// The system assigns a slightly higher priority to the thread that creates the foreground window than it does to other threads.
///
/// The foreground window can be NULL in certain circumstances, such as when a window is losing activation.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let foreground : HWND = get_foreground_window();
/// dbg!(foreground.is_null()); // may or may not be null
/// ```
pub fn get_foreground_window() -> HWND {
    fn_context!(get_foreground_window => GetForegroundWindow);
    unsafe { GetForegroundWindow() }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getshellwindow)\]
/// GetShellWindow
///
/// Retrieves a handle to the Shell's desktop window.
///
/// ### Returns
/// *   The handle of the Shell's desktop window.
/// *   If no Shell process is present, NULL.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let shell : HWND = get_shell_window();
/// assert_ne!(get_desktop_window(), shell, "These *are* different handles, but I'm not entirely sure how.");
/// ```
pub fn get_shell_window() -> HWND {
    fn_context!(get_shell_window => GetShellWindow);
    unsafe { GetShellWindow() }
}
