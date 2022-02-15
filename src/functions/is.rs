use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindow
///
/// Determines whether the specified window handle identifies an existing window.
///
/// Valid uses of this function are few and far between - windows belonging to another thread
/// or process could be destroyed immediately after this returns true, and invalidated handles
/// might suddenly spring back to life as the handle value is reused.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let valid_hwnd = get_desktop_window();
/// # let invalid_hwnd : HWND = !42 as _;
/// assert!( is_window(  valid_hwnd));
/// assert!(!is_window(invalid_hwnd));
/// assert!(!is_window(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   let _ = is_window((1usize << p) as HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(is_window => IsWindow);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindow(hwnd) != 0 },
        Err(_) => false,
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindowUnicode
///
/// Determines whether the specified window is a native Unicode window.
///
/// ### Returns
/// *   `true`  if the window's class was registered with `RegisterClassW`
/// *   `false` if the window's class was registered with `RegisterClassA`
/// *   `false` if the window isn't valid, probably?  (TryInto failed, HWND null/dangling/destroyed, ...)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let unicode_hwnd = get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// assert!( is_window_unicode(unicode_hwnd));
/// assert!(!is_window_unicode(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   let _ = is_window_unicode((1usize << p) as HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window_unicode(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(is_window_unicode => IsWindowUnicode);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindowUnicode(hwnd) != 0 },
        Err(_) => false,
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iswindow)\]
/// IsWindowVisible
///
/// Determines whether the specified window is WS_VISIBLE.
/// May return `true` even if the window is totally obscured by other windows, clipped out-of-bounds, etc.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let hwnd = get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// assert!( is_window_visible(hwnd));
/// assert!(!is_window_visible(std::ptr::null_mut()));
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   let _ = is_window_visible((1usize << p) as HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_window_visible(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(is_window_unicode => IsWindowVisible);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindowVisible(hwnd) != 0 },
        Err(_) => false,
    }
}
