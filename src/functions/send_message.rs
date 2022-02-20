use crate::*;
use crate::SMTO::SendMessageTimeOutFlags;
use winapi::um::winuser::*;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagea)\]
/// SendMessageA
///
/// Sends a message to the queue of the specified [HWnd] and waits for it to be processed.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a non-unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_message_a(thread_local_hwnd,    WM::NULL, 0, 0).unwrap();
///     send_message_a(get_desktop_window(), WM::NULL, 0, 0).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_a(!42usize as HWND, WM::NULL, 0, 0).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   ~~send_message_callback_a~~
/// *   [send_message_timeout_a]
/// *   [send_notify_message_a]
pub unsafe fn send_message_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, Error> {
    fn_context!(send_message_a => SendMessageA);
    clear_last_error();
    let lr = unsafe { SendMessageA(hwnd.into().into(), msg.into().into(), wparam, lparam) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)\]
/// SendMessageW
///
/// Sends a message to the queue of the specified [HWnd] and waits for it to be processed.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_message_w(thread_local_hwnd,    WM::NULL, 0, 0).unwrap();
///     send_message_w(get_desktop_window(), WM::NULL, 0, 0).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_w(!42usize as HWND, WM::NULL, 0, 0).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   ~~send_message_callback_w~~
/// *   [send_message_timeout_w]
/// *   [send_notify_message_w]
pub unsafe fn send_message_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, Error> {
    fn_context!(send_message_w => SendMessageW);
    clear_last_error();
    let lr = unsafe { SendMessageW(hwnd.into().into(), msg.into().into(), wparam, lparam) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

// TODO: send_message_callback[_timeout]_a
// TODO: send_message_callback[_timeout]_w

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeouta)\]
/// SendMessageTimeoutA
///
/// Sends a message to the queue of the specified [HWnd] and waits for it to be processed for up to `timeout` milliseconds.
/// If `hwnd` belongs to the current thread, it's wndproc is called directly (and timeout is ignored.)
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a non-unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::TIMEOUT]                `hwnd` didn't respond within `timeout` milliseconds.
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_message_timeout_a(thread_local_hwnd,    WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap();
///     send_message_timeout_a(get_desktop_window(), WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_timeout_a(!42usize as HWND, WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap_err(),
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_timeout_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   [send_message_a]
/// *   ~~send_message_callback_a~~
/// *   [send_notify_message_a]
pub unsafe fn send_message_timeout_a<'r>(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM, flags: impl Into<SendMessageTimeOutFlags>, timeout: u32, result: impl Into<Option<&'r mut usize>>) -> Result<LRESULT, Error> {
    fn_context!(send_message_timeout_a => SendMessageTimeoutA);
    clear_last_error();
    let lr = unsafe { SendMessageTimeoutA(hwnd.into().into(), msg.into().into(), wparam, lparam, flags.into().into(), timeout, result.into().map_or(null_mut(), |v| v)) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
/// SendMessageTimeoutW
///
/// Sends a message to the queue of the specified [HWnd] and waits for it to be processed for up to `timeout` milliseconds.
/// If `hwnd` belongs to the current thread, it's wndproc is called directly (and timeout is ignored.)
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::TIMEOUT]                `hwnd` didn't respond within `timeout` milliseconds.
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_message_timeout_w(thread_local_hwnd,    WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap();
///     send_message_timeout_w(get_desktop_window(), WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_timeout_w(!42usize as HWND, WM::NULL, 0, 0, SMTO::NORMAL, 100, None).unwrap_err(),
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_timeout_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   [send_message_w]
/// *   ~~send_message_callback_w~~
/// *   [send_notify_message_w]
pub unsafe fn send_message_timeout_w<'r>(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM, flags: impl Into<SendMessageTimeOutFlags>, timeout: u32, result: impl Into<Option<&'r mut usize>>) -> Result<LRESULT, Error> {
    fn_context!(send_message_timeout_w => SendMessageTimeoutW);
    clear_last_error();
    let lr = unsafe { SendMessageTimeoutW(hwnd.into().into(), msg.into().into(), wparam, lparam, flags.into().into(), timeout, result.into().map_or(null_mut(), |v| v)) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendnotifymessagea)\]
/// SendNotifyMessageA
///
/// Sends a message to the specified [HWnd].
///
/// *   [HWnd]s owned by the current thread will have their wndprocs invoked directly.
///     To defer to the message queue instead, use [post_message_a].
/// *   [HWnd]s owned by other threads or processes will have the message enqueued.
///     This does **not** wait for their wndprocs to execute or reply.
///     To wait instead, use [send_message_a].
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a non-unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_notify_message_a(thread_local_hwnd,    WM::NULL, 0, 0).unwrap();
///     send_notify_message_a(get_desktop_window(), WM::NULL, 0, 0).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_notify_message_a(!42usize as HWND, WM::NULL, 0, 0).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_notify_message_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   [send_message_a]
/// *   ~~send_message_callback_a~~
/// *   [send_message_timeout_a]
pub unsafe fn send_notify_message_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(send_notify_message_a => SendNotifyMessageA);
    fn_succeeded!(unsafe { SendNotifyMessageA(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendnotifymessagew)\]
/// SendNotifyMessageW
///
/// Sends a message to the specified [HWnd].
///
/// *   [HWnd]s owned by the current thread will have their wndprocs invoked directly.
///     To defer to the message queue instead, use [post_message_w].
/// *   [HWnd]s owned by other threads or processes will have the message enqueued.
///     This does **not** wait for their wndprocs to execute or reply.
///     To wait instead, use [send_message_w].
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, and remain valid until the message is processed, depending on what `msg` is and the wndproc of `hwnd`.
/// *   `hwnd` should perhaps be a unicode window?
///
/// ### Errors
/// *   [ERROR::INVALID_WINDOW_HANDLE]  If `hwnd` is invalid
/// *   [ERROR::ACCESS_DENIED]          When a message is blocked by [UIPI](https://en.wikipedia.org/wiki/User_Interface_Privilege_Isolation)
/// *   [ERROR::NOT_ENOUGH_QUOTA]       If the message queue is full.  (A message queue can contain at most 10,000 messages.)
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// # use std::ptr::*;
/// unsafe {
/// #   let thread_local_hwnd = create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap();
///     send_notify_message_w(thread_local_hwnd,    WM::NULL, 0, 0).unwrap();
///     send_notify_message_w(get_desktop_window(), WM::NULL, 0, 0).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_notify_message_w(!42usize as HWND, WM::NULL, 0, 0).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_notify_message_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   [send_message_w]
/// *   ~~send_message_callback_w~~
/// *   [send_message_timeout_w]
pub unsafe fn send_notify_message_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(send_notify_message_w => SendNotifyMessageW);
    fn_succeeded!(unsafe { SendNotifyMessageW(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}
