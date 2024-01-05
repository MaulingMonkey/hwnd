use crate::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowproca)\]
/// DefWindowProcA
///
/// Default handling of a window message.
///
/// ### Safety
/// *   **This may destroy/invalidate `hwnd`** (on e.g. `WM_CLOSE` etc.)!  See [HWnd] and [destroy_window] for rants, mitigations, and details.
/// *   `hwnd` may need to be a valid, non-unicode window handle
/// *   `wparam` and `lparam` may need to be valid pointers, depending on `msg` (and `hwnd`'s class)
///
/// ### See Also
/// *   [def_window_proc_w]
/// *   [destroy_window]
pub unsafe extern "system" fn def_window_proc_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcA(hwnd.into().into(), msg.into().into(), wparam, lparam ) }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-defwindowprocw)\]
/// DefWindowProcW
///
/// Default handling of a window message.
///
/// ### Safety
/// *   **This may destroy/invalidate `hwnd`** (on e.g. `WM_CLOSE` etc.).  See [destroy_window] for details.
/// *   `hwnd` may need to be a valid, unicode window handle
/// *   `wparam` and `lparam` may need to be valid pointers, depending on `msg` (and `hwnd`'s class)
///
/// ### See Also
/// *   [def_window_proc_a]
/// *   [destroy_window]
pub unsafe extern "system" fn def_window_proc_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe { DefWindowProcW(hwnd.into().into(), msg.into().into(), wparam, lparam ) }
}
