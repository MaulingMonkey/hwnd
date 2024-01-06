use crate::*;
use crate::SMTO::SendMessageTimeOutFlags;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagea)\]
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
/// #   send_message_a(thread_local_hwnd,    WM::SETTEXT, 0, abistr::cstr!("asdf").as_ptr() as _).unwrap(); // IDK if this is the right string width or not
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
/// *   [send_message_callback_a]
/// *   [send_message_timeout_a]
/// *   [send_notify_message_a]
pub unsafe fn send_message_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, Error> {
    fn_context!(send_message_a => SendMessageA);
    clear_last_error();
    let lr = unsafe { SendMessageA(hwnd.into().into(), msg.into().into(), wparam, lparam) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagew)\]
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
/// #   send_message_w(thread_local_hwnd,    WM::SETTEXT, 0, abistr::cstr16!("asdf").as_ptr() as _).unwrap(); // IDK if this is the right string width or not
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
/// *   [send_message_callback_w]
/// *   [send_message_timeout_w]
/// *   [send_notify_message_w]
pub unsafe fn send_message_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<LRESULT, Error> {
    fn_context!(send_message_w => SendMessageW);
    clear_last_error();
    let lr = unsafe { SendMessageW(hwnd.into().into(), msg.into().into(), wparam, lparam) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagecallbacka)\]
/// SendMessageCallbackA
///
/// Sends a message to the specified [HWnd].
///
/// If [HWnd::BROADCAST] is specified, the message is sent to all top-level windows in the system,
/// including disabled or invisible unowned windows, overlapped windows, and pop-up windows;
/// but the message is not sent to child windows.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, depending on `msg` and the windows involved.
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
/// # use std::sync::atomic::*;
/// # let thread_local_hwnd = unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// unsafe {
///     static CALLS : AtomicUsize = AtomicUsize::new(0);
///
///     unsafe extern "system" fn never_called(_: HWnd, _: WM32, _: usize, _: LRESULT) { unreachable!() }
///     unsafe extern "system" fn on_null(_hwnd: HWnd, msg: WM32, data: usize, lresult: LRESULT) {
///         assert_eq!(msg,     WM::NULL);
///         assert_eq!(data,    0xDA7A  );
///         assert_eq!(lresult, 0       );
///         CALLS.fetch_add(1, Ordering::Relaxed);
///     }
///
///     send_message_callback_a(thread_local_hwnd,      WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///     send_message_callback_a(get_desktop_window(),   WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///     send_message_callback_a(HWnd::BROADCAST,        WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_callback_a(!9usize as HWND, WM::NULL, 0, 0, on_null, 0xDA7A).unwrap_err()
///     );
///
///     assert_eq!(
///         ERROR::MESSAGE_SYNC_ONLY,
///         send_message_callback_a(
///             thread_local_hwnd, WM::SETTEXT, 0,
///             abistr::cstr!("asdf").as_ptr() as _, // *guessing* that WM_SETTEXT takes _TCHAR[]
///             never_called, 0,
///         ).unwrap_err()
///     );
///
///     // pump message loop a bit to allow `on_null` to be run
///     // in response  to other processes handling WM::NULL
///     let timeout = std::time::Instant::now() + std::time::Duration::from_secs(1);
///     while std::time::Instant::now() < timeout {
///         if let Some(msg) = peek_message_w(HWnd::NULL, 0, 0, PM::REMOVE) {
///             translate_message(&msg);
///             let _ = unsafe { dispatch_message_w(&msg) };
///         }
///     }
///
///     assert!(CALLS.load(Ordering::Relaxed) >= 3);
/// }
/// ```
///
/// ### See Also
/// *   [send_message_callback_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   [send_message_a]
/// *   [send_message_timeout_a]
/// *   [send_notify_message_a]
pub unsafe fn send_message_callback_a(
    hwnd:               impl Into<HWnd>,
    msg:                impl Into<WM32>,
    wparam:             WPARAM,
    lparam:             LPARAM,
    result_callback:    unsafe extern "system" fn (hwnd: HWnd, msg: WM32, data: usize, lr: LRESULT),
    data:               usize,
) -> Result<(), Error> {
    fn_context!(send_message_callback_a => SendMessageCallbackA);
    fn_succeeded!(unsafe { SendMessageCallbackA(hwnd.into().into(), msg.into().into(), wparam, lparam, Some(std::mem::transmute(result_callback)), data) })
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagecallbackw)\]
/// SendMessageCallbackW
///
/// Sends a message to the specified [HWnd].
///
/// If [HWnd::BROADCAST] is specified, the message is sent to all top-level windows in the system,
/// including disabled or invisible unowned windows, overlapped windows, and pop-up windows;
/// but the message is not sent to child windows.
///
/// ### Safety
/// *   `wparam` and `lparam` may need to be valid pointers, depending on `msg` and the windows involved.
/// *   `hwnd` should perhaps be a unicode window?
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
/// # use std::sync::atomic::*;
/// # let thread_local_hwnd = unsafe { create_window_w(abistr::cstr16!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
/// unsafe {
///     static CALLS : AtomicUsize = AtomicUsize::new(0);
///
///     unsafe extern "system" fn never_called(_: HWnd, _: WM32, _: usize, _: LRESULT) { unreachable!() }
///     unsafe extern "system" fn on_null(_hwnd: HWnd, msg: WM32, data: usize, lresult: LRESULT) {
///         assert_eq!(msg,     WM::NULL);
///         assert_eq!(data,    0xDA7A  );
///         assert_eq!(lresult, 0       );
///         CALLS.fetch_add(1, Ordering::Relaxed);
///     }
///
///     send_message_callback_w(thread_local_hwnd,      WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///     send_message_callback_w(get_desktop_window(),   WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///     send_message_callback_w(HWnd::BROADCAST,        WM::NULL, 0, 0, on_null, 0xDA7A).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_callback_w(!9usize as HWND, WM::NULL, 0, 0, on_null, 0xDA7A).unwrap_err()
///     );
///
///     assert_eq!(
///         ERROR::MESSAGE_SYNC_ONLY,
///         send_message_callback_w(
///             thread_local_hwnd, WM::SETTEXT, 0,
///             abistr::cstr16!("asdf").as_ptr() as _, // *guessing* that WM_SETTEXT takes _TCHAR[]
///             never_called, 0,
///         ).unwrap_err()
///     );
///
///     // pump message loop a bit to allow `on_null` to be run
///     // in response  to other processes handling WM::NULL
///     let timeout = std::time::Instant::now() + std::time::Duration::from_secs(1);
///     while std::time::Instant::now() < timeout {
///         if let Some(msg) = peek_message_w(HWnd::NULL, 0, 0, PM::REMOVE) {
///             translate_message(&msg);
///             let _ = unsafe { dispatch_message_w(&msg) };
///         }
///     }
///
///     assert!(CALLS.load(Ordering::Relaxed) >= 3);
/// }
/// ```
///
/// ### See Also
/// *   [send_message_callback_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   [send_message_w]
/// *   [send_message_timeout_w]
/// *   [send_notify_message_w]
pub unsafe fn send_message_callback_w(
    hwnd:               impl Into<HWnd>,
    msg:                impl Into<WM32>,
    wparam:             WPARAM,
    lparam:             LPARAM,
    result_callback:    unsafe extern "system" fn (hwnd: HWnd, msg: WM32, data: usize, lr: LRESULT),
    data:               usize,
) -> Result<(), Error> {
    fn_context!(send_message_callback_w => SendMessageCallbackW);
    fn_succeeded!(unsafe { SendMessageCallbackW(hwnd.into().into(), msg.into().into(), wparam, lparam, Some(std::mem::transmute(result_callback)), data) })
}

// TODO: send_message_callback_timeout_a using Fn() and generational indicies or something?
// TODO: send_message_callback_timeout_w using Fn() and generational indicies or something?

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeouta)\]
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
/// #   send_message_timeout_a(thread_local_hwnd,    WM::SETTEXT, 0, abistr::cstr!("asdf").as_ptr() as _, SMTO::NORMAL, 100).unwrap();
///     send_message_timeout_a(thread_local_hwnd,    WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap();
///     send_message_timeout_a(get_desktop_window(), WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_timeout_a(!42usize as HWND, WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap_err(),
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_timeout_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   [send_message_a]
/// *   [send_message_callback_a]
/// *   [send_notify_message_a]
pub unsafe fn send_message_timeout_a<'r>(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM, flags: impl Into<SendMessageTimeOutFlags>, timeout: u32) -> Result<LRESULT, Error> {
    fn_context!(send_message_timeout_a => SendMessageTimeoutA);
    clear_last_error();
    let mut lresult = 0;
    fn_succeeded!(unsafe { SendMessageTimeoutA(hwnd.into().into(), msg.into().into(), wparam, lparam, flags.into().into(), timeout, &mut lresult) } != 0)?;
    Ok(lresult as _)
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
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
/// #   send_message_timeout_w(thread_local_hwnd,    WM::SETTEXT, 0, abistr::cstr16!("asdf").as_ptr() as _, SMTO::NORMAL, 100).unwrap();
///     send_message_timeout_w(thread_local_hwnd,    WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap();
///     send_message_timeout_w(get_desktop_window(), WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap();
///
///     assert_eq!(
///         ERROR::INVALID_WINDOW_HANDLE,
///         send_message_timeout_w(!42usize as HWND, WM::NULL, 0, 0, SMTO::NORMAL, 100).unwrap_err(),
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_message_timeout_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   [send_message_w]
/// *   [send_message_callback_w]
/// *   [send_notify_message_w]
pub unsafe fn send_message_timeout_w<'r>(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM, flags: impl Into<SendMessageTimeOutFlags>, timeout: u32) -> Result<LRESULT, Error> {
    fn_context!(send_message_timeout_w => SendMessageTimeoutW);
    clear_last_error();
    let mut lresult = 0;
    fn_succeeded!(unsafe { SendMessageTimeoutW(hwnd.into().into(), msg.into().into(), wparam, lparam, flags.into().into(), timeout, &mut lresult) } != 0)?;
    Ok(lresult as _)
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendnotifymessagea)\]
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
/// *   [ERROR::MESSAGE_SYNC_ONLY]      If `msg` is a system message to be handled syncronously (common for messages with pointers.)<br>
///                                     This occurs even if `hwnd` belongs to the current thread.
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
///
///     let text = abistr::cstr!("asdf").as_ptr(); // *guessing* WM::SETTEXT takes TCHAR[]
///     assert_eq!(
///         ERROR::MESSAGE_SYNC_ONLY,
///         send_notify_message_a(thread_local_hwnd, WM::SETTEXT, 0, text as _).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_notify_message_w] (wide variant)
/// *   [post_message_a]
/// *   [reply_message]
/// *   [send_message_a]
/// *   [send_message_callback_a]
/// *   [send_message_timeout_a]
pub unsafe fn send_notify_message_a(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(send_notify_message_a => SendNotifyMessageA);
    fn_succeeded!(unsafe { SendNotifyMessageA(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendnotifymessagew)\]
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
/// *   [ERROR::MESSAGE_SYNC_ONLY]      If `msg` is a system message to be handled syncronously (common for messages with pointers.)<br>
///                                     This occurs even if `hwnd` belongs to the current thread.
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
///
///     let text = abistr::cstr16!("asdf").as_ptr(); // *guessing* WM::SETTEXT takes TCHAR[]
///     assert_eq!(
///         ERROR::MESSAGE_SYNC_ONLY,
///         send_notify_message_w(thread_local_hwnd, WM::SETTEXT, 0, text as _).unwrap_err()
///     );
/// }
/// ```
///
/// ### See Also
/// *   [send_notify_message_a] (narrow variant)
/// *   [post_message_w]
/// *   [reply_message]
/// *   [send_message_w]
/// *   [send_message_callback_w]
/// *   [send_message_timeout_w]
pub unsafe fn send_notify_message_w(hwnd: impl Into<HWnd>, msg: impl Into<WM32>, wparam: WPARAM, lparam: LPARAM) -> Result<(), Error> {
    fn_context!(send_notify_message_w => SendNotifyMessageW);
    fn_succeeded!(unsafe { SendNotifyMessageW(hwnd.into().into(), msg.into().into(), wparam, lparam) })
}
