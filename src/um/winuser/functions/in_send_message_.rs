use crate::*;
use ISMEX::InSendMessageExFlags;
use winapi::um::winuser::*;
use std::ptr::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessage)\]
/// InSendMessage
///
/// Determines whether the current window procedure is processing a message that was sent from another thread (in the same process or a different process) by a call to [send_message](crate::send_message_w).
///
/// | Sender                        | [`in_send_message`]   | [`in_send_message_ex`]    |
/// | ------------------------------|:---------------------:| --------------------------|
/// | [`post_thread_message_w`]     | `false`               | <code>[ISMEX]::[NOSEND](ISMEX::NOSEND)</code> (not that any [`WndProc`] executes for context)
/// | [`post_message_w`]            | `false`               | <code>[ISMEX]::[NOSEND](ISMEX::NOSEND)</code>
/// | [`send_message_w`]            | `true`                | <code>[ISMEX]::[SEND](ISMEX::SEND)</code>
/// | [`send_message_callback_w`]   | `true`                | <code>[ISMEX]::[CALLBACK](ISMEX::CALLBACK)</code>
/// | [`send_notify_message_w`]     | `true`                | <code>[ISMEX]::[NOTIFY](ISMEX::NOTIFY)</code>
/// | [`send_message_timeout_w`]    | `true`                | <code>[ISMEX]::[SEND](ISMEX::SEND)</code>
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// assert!(!in_send_message());
///
/// // Equivalent expressions:
/// assert_eq!(
///     in_send_message(),
///     in_send_message_ex(None) != ISMEX::NOSEND,
/// );
/// ```
///
/// ### See Also
/// *   [`in_send_message_ex`]  &mdash; More detailed information
pub fn in_send_message() -> bool {
    fn_context!(in_send_message => InSendMessage);
    unsafe { InSendMessage() != 0 }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)\]
/// InSendMessageEx
///
/// Determines whether the current window procedure is processing a message that was sent from another thread (in the same process or a different process).
///
/// | Sender                        | [`in_send_message`]   | [`in_send_message_ex`]    |
/// | ------------------------------|:---------------------:| --------------------------|
/// | [`post_thread_message_w`]     | `false`               | <code>[ISMEX]::[NOSEND](ISMEX::NOSEND)</code> (not that any [`WndProc`] executes for context)
/// | [`post_message_w`]            | `false`               | <code>[ISMEX]::[NOSEND](ISMEX::NOSEND)</code>
/// | [`send_message_w`]            | `true`                | <code>[ISMEX]::[SEND](ISMEX::SEND)</code>
/// | [`send_message_callback_w`]   | `true`                | <code>[ISMEX]::[CALLBACK](ISMEX::CALLBACK)</code>
/// | [`send_notify_message_w`]     | `true`                | <code>[ISMEX]::[NOTIFY](ISMEX::NOTIFY)</code>
/// | [`send_message_timeout_w`]    | `true`                | <code>[ISMEX]::[SEND](ISMEX::SEND)</code>
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// # use winresult::*;
/// #
/// # assert_eq!(ISMEX::NOSEND, in_send_message_ex(None));
/// #
/// // Reasonable usage:
/// const WM_CUSTOM_MESSAGE_THAT_EXPECTS_POINTERS : WM32 = WM::USER(0); // e.g. wparam ≈ size_t* ?
/// unsafe extern "system" fn wnd_proc(hwnd: HWnd, msg: WM32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
///     if msg == WM_CUSTOM_MESSAGE_THAT_EXPECTS_POINTERS {
///         if in_send_message_ex(None) != ISMEX::SEND { return ERROR::MESSAGE_SYNC_ONLY.to_u32() as _ }
///         // ...use wparam/lparam...
///     }
///     def_window_proc_w(hwnd, msg, wparam, lparam)
/// }
///
/// // Equivalent expressions (not super useful IMO:)
/// assert_eq!(
///     in_send_message(),
///     in_send_message_ex(None) != ISMEX::NOSEND,
/// );
/// ```
///
/// ### See Also
/// *   [`in_send_message`]     &mdash; Simpler alternative to <code>[in_send_message_ex]\([None]\) != [ISMEX]::[NOSEND](ISMEX::NOSEND)</code>
/// *   [`ISMEX`]               &mdash; Constants [`in_send_message_ex`] might return.
pub fn in_send_message_ex(_reserved: impl Into<Option<std::convert::Infallible>>) -> InSendMessageExFlags {
    fn_context!(in_send_message_ex => InSendMessageEx);
    InSendMessageExFlags::from(unsafe { InSendMessageEx(null_mut()) })
}

#[test] fn test_in_send_message_vs_ex() {
    use crate::*;
    use abistr::*;

    const WM_EXAMPLE : WM32 = WM::APP(0);
    let hinstance = get_module_handle_entry_exe().unwrap();
    let name = cstr16!("test_in_send_message");

    let class = unsafe { register_class_w(&WndClassW { wnd_proc: Some(window_proc), hinstance, class_name: name.into(), .. Default::default() }) }.unwrap();
    let hwnd  = unsafe { create_window_ex_w(0, class, name, 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, HWnd::MESSAGE, null_mut(), hinstance, null_mut()) }.unwrap();
    let main_thread = get_current_thread_id();

    assert_eq!((),     unsafe { post_thread_message_w(main_thread, WM_EXAMPLE, 0, 1                       ) }.unwrap()); // queued, bypasses wndproc
    assert_eq!((),     unsafe { post_message_w          (hwnd, WM_EXAMPLE, 1, 2                           ) }.unwrap()); // queued
    assert_eq!(3+4+0,  unsafe { send_message_w          (hwnd, WM_EXAMPLE, 3, 4                           ) }.unwrap()); // sync
    assert_eq!((),     unsafe { send_message_callback_w (hwnd, WM_EXAMPLE, 5, 6, wm_user0_callback, 5+6+0 ) }.unwrap()); // sync
    assert_eq!((),     unsafe { send_notify_message_w   (hwnd, WM_EXAMPLE, 7, 8                           ) }.unwrap()); // sync (same thread)
    assert_eq!(9+10+0, unsafe { send_message_timeout_w  (hwnd, WM_EXAMPLE, 9, 10, SMTO::NORMAL, 100       ) }.unwrap()); // sync

    let thread = std::thread::spawn(move || {
        assert_eq!((),     unsafe { post_thread_message_w(main_thread, WM_EXAMPLE, 0, 1                       ) }.unwrap()); // queued, bypasses wndproc
        assert_eq!((),     unsafe { post_message_w          (hwnd, WM_EXAMPLE, 1, 2                           ) }.unwrap()); // queued
        assert_eq!(3+4+1,  unsafe { send_message_w          (hwnd, WM_EXAMPLE, 3, 4                           ) }.unwrap()); // sync
        assert_eq!((),     unsafe { send_message_callback_w (hwnd, WM_EXAMPLE, 5, 6, wm_user0_callback, 5+6+1 ) }.unwrap()); // sync
        assert_eq!((),     unsafe { send_notify_message_w   (hwnd, WM_EXAMPLE, 7, 8                           ) }.unwrap()); // queued (different thread)
        assert_eq!(9+10+1, unsafe { send_message_timeout_w  (hwnd, WM_EXAMPLE, 9, 10, SMTO::NORMAL, 100       ) }.unwrap()); // sync
        unsafe { post_message_w(hwnd, WM::QUIT, 0, 0) }.unwrap();
    });

    let mut msg = Msg::zeroed();
    while get_message_w(&mut msg, None, 0, 0).unwrap() {
        translate_message(&msg);
        unsafe { dispatch_message_w(&mut msg) }.unwrap();
    }

    thread.join().unwrap();

    unsafe extern "system" fn window_proc(hwnd: HWnd, msg: WM32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
        match msg {
            WM_EXAMPLE => {
                let result = wparam as isize + lparam;
                dbg!(("WM_EXAMPLE", wparam, lparam, result, in_send_message_ex(None)));
                assert!(result & 1 != 0, "a + b was supposed to be odd");

                assert_eq!(in_send_message(), in_send_message_ex(None) != ISMEX::NOSEND, "before reply");
                let _ = reply_message(result + 1);
                assert_eq!(in_send_message(), in_send_message_ex(None) != ISMEX::NOSEND, "after reply");

                result + 0
            },
            _ => unsafe { def_window_proc_w(hwnd, msg, wparam, lparam) },
        }
    }

    unsafe extern "system" fn wm_user0_callback(hwnd: HWnd, msg: WM32, expected: usize, lr: LRESULT) {
        assert!(expected == 1 || !hwnd.is_null());
        assert_eq!(msg, WM_EXAMPLE);
        assert_eq!(expected as LRESULT, lr);
    }
}
