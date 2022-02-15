use crate::*;

use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ischild)\]
/// IsChild
///
/// Determines whether a window is a child window or descendant window of a specified parent window.
/// A child window is the direct descendant of a specified parent window if that parent window is in the chain of parent windows;
/// the chain of parent windows leads from the original overlapped or pop-up window to the child window.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use std::ptr::*;
/// assert!(!is_child(get_desktop_window(), null_mut()));
/// # let _ = is_child(get_desktop_window(), !42usize as HWND);
/// # assert!(!is_child(get_desktop_window(), get_desktop_window()));
/// # assert!(!is_child(get_desktop_window(), get_shell_window()));
/// ```
pub fn is_child(parent: impl TryInto<HWND>, child: impl TryInto<HWND>) -> bool {
    fn_context!(is_child => IsChild);
    let parent = if let Ok(h) = parent.try_into() { h } else { return false };
    let child  = if let Ok(h) = child .try_into() { h } else { return false };
    unsafe { IsChild(parent, child) != 0 }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)\]
/// IsGUIThread\(FALSE\)
///
/// Determines whether the calling thread is already a GUI thread.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # if false {
/// debug_assert!(!is_gui_thread(), "don't do I/O on the GUI thread");
/// let huge_file = std::fs::read("...").unwrap();
/// # }
/// #
/// # let _ = is_gui_thread(); // unit tests likely converted this to a GUI thread, but it's not guaranteed.
/// # std::thread::spawn(|| assert!(!is_gui_thread())).join();
/// ```
///
/// ### See Also
/// *   [convert_to_gui_thread]
pub fn is_gui_thread() -> bool {
    fn_context!(is_gui_thread => IsGUIThread);
    unsafe { IsGUIThread(0) != 0 }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isguithread)\]
/// IsGUIThread\(TRUE\)
///
/// Convert the thread to a GUI thread.
///
/// ### Errors
/// *   [ERROR::NOT_ENOUGH_MEMORY]
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let _ = is_gui_thread(); // unit tests likely converted this to a GUI thread, but it's not guaranteed.
/// std::thread::spawn(|| {
///     assert!(!is_gui_thread());
///     convert_to_gui_thread().unwrap();
///     assert!(is_gui_thread());
/// }).join();
/// ```
///
/// ### See Also
/// *   [is_gui_thread]
pub fn convert_to_gui_thread() -> Result<(), Error> {
    fn_context!(convert_to_gui_thread => IsGUIThread);
    match unsafe { IsGUIThread(1) } {
        0 => fn_succeeded!(0),
        1 => fn_succeeded!(1),
        // Docs state "..., IsGUIThread returns ERROR_NOT_ENOUGH_MEMORY".
        // I don't know if that's accurate or not, but we'll assume it is if we got something other than FALSE or TRUE.
        n => fn_err!(n as u32),
    }
}

// DO NOT DEFINE: IsHungAppWindow
// "[This function is not intended for general use. It may be altered or unavailable in subsequent versions of Windows.]"
// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-ishungappwindow

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)\]
/// IsIconic
///
/// Determines whether a window is minimized.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let hwnd = get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// let vscode = get_foreground_window(); // as if I'd have anything else focused
/// # if false {
/// assert!(is_iconic(vscode)); // as if I'd ever run vscode non-maximized
/// # }
///
/// assert!(!is_iconic(std::ptr::null_mut()));
/// assert!(!is_iconic(get_desktop_window()));
/// #
/// # assert!(!is_iconic(get_shell_window()));
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   let _ = is_iconic((1usize << p) as HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_iconic(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(is_iconic => IsIconic);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsIconic(hwnd) != 0 },
        Err(_) => false,
    }
}

// DO NOT DEFINE: IsProcessDPIAware
// "[IsProcessDPIAware is available for use in the operating systems specified in the Requirements section. It may be altered or unavailable in subsequent versions. Instead, use GetProcessDPIAwareness.]"
// https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-isprocessdpiaware

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
    fn_context!(is_window_visible => IsWindowVisible);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsWindowVisible(hwnd) != 0 },
        Err(_) => false,
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-iszoomed)\]
/// IsZoomed
///
/// Determines whether a window is maximized.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # let hwnd = get_desktop_window(); // TODO: replace with an explicitly created unicode hwnd
/// let vscode = get_foreground_window(); // as if I'd have anything else focused
/// # if false {
/// assert!(is_zoomed(vscode)); // as if I'd ever run vscode non-maximized
/// # }
///
/// assert!(!is_zoomed(std::ptr::null_mut()));
/// assert!(!is_zoomed(get_desktop_window()));
/// #
/// # assert!(!is_zoomed(get_shell_window()));
/// # for p in 0 .. 8 * std::mem::size_of::<HWND>() {
/// #   let _ = is_zoomed((1usize << p) as HWND); // shouldn't crash
/// # }
/// ```
#[must_use] pub fn is_zoomed(hwnd: impl TryInto<HWND>) -> bool {
    fn_context!(is_zoomed => IsZoomed);
    match hwnd.try_into() {
        Ok(hwnd) => unsafe { IsZoomed(hwnd) != 0 },
        Err(_) => false,
    }
}
