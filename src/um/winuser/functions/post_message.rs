use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagea)\]
/// PostMessageA
///
/// Posts a message in the message queue associated with the thread that created the specified [HWnd], and returns without waiting for the thread to process the message.
///
/// *   [HWnd::BROADCAST] will post the message to all top-level windows in the system,
///     including disabled or invisible unowned windows, overlapped windows, and pop-up windows.
///     The message is not posted to child windows.
/// *   [HWnd::NULL] will behave like [post_thread_message_a], posting to the current thread.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a non-unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
/// *   [ERROR::MESSAGE_SYNC_ONLY]      If `msg` is a system message to be handled syncronously (common for messages with pointers.)<br>
///                                     This occurs even if `hwnd` belongs to the current thread.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// unsafe { post_message_a(hwnd, WM::NULL, 0, 0) }.unwrap();
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, unsafe { post_message_a(!42usize as HWND, WM::NULL, 0, 0) }.unwrap_err());
/// assert_eq!(ERROR::MESSAGE_SYNC_ONLY, unsafe { post_message_a(hwnd, WM::SETTEXT, 0, 0) }.unwrap_err());
/// #
/// # let r = std::thread::spawn(||{
/// #   let hwnd = unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// #   for _ in 0 .. 11_000 { drop(unsafe { post_message_a(hwnd, WM::NULL, 0, 0) }) }
/// #   unsafe { post_message_a(hwnd, WM::NULL, 0, 0) }
/// # }).join().unwrap();
/// # assert_eq!(ERROR::NOT_ENOUGH_QUOTA, r.unwrap_err());
/// ```
pub unsafe fn post_message_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(post_message_a => PostMessageA);
    fn_succeeded!(unsafe { PostMessageA(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagew)\]
/// PostMessageW
///
/// Posts a message in the message queue associated with the thread that created the specified [HWnd], and returns without waiting for the thread to process the message.
///
/// *   [HWnd::BROADCAST] will post the message to all top-level windows in the system,
///     including disabled or invisible unowned windows, overlapped windows, and pop-up windows.
///     The message is not posted to child windows.
/// *   [HWnd::NULL] will behave like [post_thread_message_w], posting to the current thread.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
/// *   [ERROR::MESSAGE_SYNC_ONLY]      If `msg` is a system message that can only be dispatched syncronously (typically wparam/lparam are expected to be pointers)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// # let hwnd = unsafe { create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// unsafe { post_message_w(hwnd, WM::NULL, 0, 0) }.unwrap();
/// assert_eq!(ERROR::INVALID_WINDOW_HANDLE, unsafe { post_message_w(!42usize as HWND, WM::NULL, 0, 0) }.unwrap_err());
/// assert_eq!(ERROR::MESSAGE_SYNC_ONLY, unsafe { post_message_w(hwnd, WM::SETTEXT, 0, 0) }.unwrap_err());
/// #
/// # let r = std::thread::spawn(||{
/// #   let hwnd = unsafe { create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// #   for _ in 0 .. 11_000 { drop(unsafe { post_message_w(hwnd, WM::NULL, 0, 0) }) }
/// #   unsafe { post_message_w(hwnd, WM::NULL, 0, 0) }
/// # }).join().unwrap();
/// # assert_eq!(ERROR::NOT_ENOUGH_QUOTA, r.unwrap_err());
/// ```
pub unsafe fn post_message_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(post_message_w => PostMessageW);
    fn_succeeded!(unsafe { PostMessageW(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagea)\]
/// PostThreadMessageA
///
/// Posts a message to the message queue of the specified thread. It returns without waiting for the thread to process the message.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndprocs of `thread`.
///
/// ### Errors
/// *   [ERROR::INVALID_THREAD_ID]  If `thread` is invalid
/// *   [ERROR::ACCESS_DENIED]      When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]   If the message queue is full.  (A message queue can contain at most 10,000 messages.)
/// *   [ERROR::MESSAGE_SYNC_ONLY]  If `msg` is a system message to be handled syncronously (common for messages with pointers.)<br>
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe { post_thread_message_a(get_current_thread_id(), WM::NULL, 0, 0) }.unwrap();
/// assert_eq!(ERROR::INVALID_THREAD_ID, unsafe { post_thread_message_a(0, WM::NULL, 0, 0) }.unwrap_err());
/// assert_eq!(ERROR::MESSAGE_SYNC_ONLY, unsafe { post_thread_message_a(get_current_thread_id(), WM::SETTEXT, 0, 0) }.unwrap_err());
/// #
/// # let r = std::thread::spawn(||{
/// #   let thread = get_current_thread_id();
/// #   for _ in 0 .. 11_000 { drop(unsafe { post_thread_message_a(thread, WM::NULL, 0, 0) }) }
/// #   unsafe { post_thread_message_a(thread, WM::NULL, 0, 0) }
/// # }).join().unwrap();
/// # assert_eq!(ERROR::NOT_ENOUGH_QUOTA, r.unwrap_err());
/// ```
pub unsafe fn post_thread_message_a(thread: u32, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(post_thread_message_a => PostThreadMessageA);
    fn_succeeded!(unsafe { PostThreadMessageA(thread, msg.into().into(), wparam, lparam) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postthreadmessagew)\]
/// PostThreadMessageW
///
/// Posts a message to the message queue of the specified thread. It returns without waiting for the thread to process the message.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndprocs of `thread`.
///
/// ### Errors
/// *   [ERROR::INVALID_THREAD_ID]  If `thread` is invalid
/// *   [ERROR::ACCESS_DENIED]      When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]   If the message queue is full.  (A message queue can contain at most 10,000 messages.)
/// *   [ERROR::MESSAGE_SYNC_ONLY]  If `msg` is a system message to be handled syncronously (common for messages with pointers.)<br>
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe { post_thread_message_w(get_current_thread_id(), WM::NULL, 0, 0) }.unwrap();
/// assert_eq!(ERROR::INVALID_THREAD_ID, unsafe { post_thread_message_w(0, WM::NULL, 0, 0) }.unwrap_err());
/// assert_eq!(ERROR::MESSAGE_SYNC_ONLY, unsafe { post_thread_message_w(get_current_thread_id(), WM::SETTEXT, 0, 0) }.unwrap_err());
/// #
/// # let r = std::thread::spawn(||{
/// #   let thread = get_current_thread_id();
/// #   for _ in 0 .. 11_000 { drop(unsafe { post_thread_message_w(thread, WM::NULL, 0, 0) }) }
/// #   unsafe { post_thread_message_w(thread, WM::NULL, 0, 0) }
/// # }).join().unwrap();
/// # assert_eq!(ERROR::NOT_ENOUGH_QUOTA, r.unwrap_err());
/// ```
pub unsafe fn post_thread_message_w(thread: u32, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(post_thread_message_w => PostThreadMessageW);
    fn_succeeded!(unsafe { PostThreadMessageW(thread, msg.into().into(), wparam, lparam) })
}

// XXX: post_app_message_a appears to be a macro alias for post_thread_message_a? also undocumented as heck, so let's not expose it
// XXX: post_app_message_w appears to be a macro alias for post_thread_message_w? also undocumented as heck, so let's not expose it

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postquitmessage)\]
/// PostQuitMessage
///
/// Posts a [WM::QUIT] message (with `wparam` = `exit_code`) to the thread's message queue and returns immediately;
/// the function simply indicates to the system that the thread is requesting to quit at some time in the future.
///
/// When the thread retrieves the [WM::QUIT] message from its message queue, it should exit its message loop and return control to the system.
/// The exit value returned to the system must be the `wparam` parameter of the [WM::QUIT] message.
///
/// Typically used in response to [WM::DESTROY]ing the last or "main" window.<br>
/// Used by default in response to [WM::DESTROY] by [def_window_proc](def_window_proc_w).
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// post_quit_message(0);
/// ```
pub fn post_quit_message(exit_code: i32) {
    fn_context!(post_quit_message => PostQuitMessage);
    unsafe { PostQuitMessage(exit_code) }
}
