use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)\]
/// GetWindowThreadProcessId
///
/// Retrieves the identifier of the thread that created the specified window.
///
/// ### Returns
/// *   Ok(thread_id)
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let hwnd = get_desktop_window();
/// let thread = get_window_thread_id(hwnd).unwrap();
/// let hwnd_belongs_to_this_thread = thread == get_current_thread_id();
/// # assert!(!hwnd_belongs_to_this_thread, "desktop doesn't belong to us!");
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_thread_id(null_mut()).unwrap_err());
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   if let Err(e) = get_window_thread_id((1usize << p) as HWND) { // shouldn't crash
/// #       assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e);
/// #   }
/// # }
/// ```
///
/// ### See Also
/// *   [get_window_thread_process_id]
/// *   [get_window_process_id]
#[must_use] pub fn get_window_thread_id(hwnd: impl Into<HWND>) -> Result<u32, Error> {
    fn_context!(get_window_thread_id => GetWindowThreadProcessId);
    let hwnd = hwnd.into();
    let tid = unsafe { GetWindowThreadProcessId(hwnd, std::ptr::null_mut()) };
    if tid != 0 { Ok(tid) } else { Err(fn_error_gle!()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)\]
/// GetWindowThreadProcessId
///
/// Retrieves the identifier of the thread that created the specified window.
///
/// ### Returns
/// *   Ok(thread_id)
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let hwnd = get_desktop_window();
/// let process = get_window_process_id(hwnd).unwrap();
/// let hwnd_belongs_to_this_process = process == get_current_process_id();
/// # assert!(!hwnd_belongs_to_this_process, "desktop doesn't belong to us!");
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_process_id(null_mut()).unwrap_err());
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   if let Err(e) = get_window_process_id((1usize << p) as HWND) { // shouldn't crash
/// #       assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e);
/// #   }
/// # }
/// ```
///
/// ### See Also
/// *   [get_window_thread_id]
/// *   [get_window_thread_process_id]
#[must_use] pub fn get_window_process_id(hwnd: impl Into<HWND>) -> Result<u32, Error> {
    fn_context!(get_window_process_id => GetWindowThreadProcessId);
    let hwnd = hwnd.into();
    let mut pid = 0;
    let tid = unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if tid != 0 { Ok(pid) } else { Err(fn_error_gle!()) }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getwindowthreadprocessid)\]
/// GetWindowThreadProcessId
///
/// Retrieves the identifier of the thread that created the specified window and the identifier of the process that created the window.
///
/// ### Returns
/// *   Ok((thread_id, process_id))
/// *   [ERROR::INVALID_WINDOW_HANDLE]
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// # let hwnd = get_desktop_window();
/// let (thread, process) = get_window_thread_process_id(hwnd).unwrap();
/// let hwnd_belongs_to_this_thread = thread == get_current_thread_id();
/// # assert!(!hwnd_belongs_to_this_thread, "desktop doesn't belong to us!");
/// #
/// # assert_eq!(ERROR::INVALID_WINDOW_HANDLE, get_window_thread_process_id(null_mut()).unwrap_err());
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   if let Err(e) = get_window_thread_process_id((1usize << p) as HWND) { // shouldn't crash
/// #       assert_eq!(ERROR::INVALID_WINDOW_HANDLE, e);
/// #   }
/// # }
/// ```
///
/// ### See Also
/// *   [get_window_process_id]
/// *   [get_window_thread_id]
#[must_use] pub fn get_window_thread_process_id(hwnd: impl Into<HWND>) -> Result<(u32, u32), Error> {
    fn_context!(get_window_thread_process_id => GetWindowThreadProcessId);
    let hwnd = hwnd.into();
    let mut pid = 0;
    let tid = unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if tid != 0 { Ok((tid, pid)) } else { Err(fn_error_gle!()) }
}

// TODO: test coverage with process/thread local windows
