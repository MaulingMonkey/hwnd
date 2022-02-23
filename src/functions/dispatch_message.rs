use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagea)\]
/// DispatchMessageA
///
/// Dispatches a message to a window procedure. It is typically used to dispatch a message retrieved by the [get_message](get_message_a) function.
///
/// ### Safety
/// *   `msg.wparam` and `msg.lparam` may need to be valid pointers, depending on the value of `msg.message` and the wndproc of `msg.hwnd`.
/// *   `msg.hwnd` should perhaps be a unicode window?
///
/// ### Returns
/// The return value specifies the value returned by the window procedure.
/// Although its meaning depends on the message being dispatched, the return value generally is ignored.
///
/// ### Errors
/// These are typically ignored!
/// *   [ERROR::INVALID_WINDOW_HANDLE]  if `msg.hwnd` was invalid, **or if the wndproc destroyed the window.**<br>
///     Common for WM::SYSCOMMAND (Alt+F4), WM::NCLBUTTONDOWN (clicked X), etc.
/// *   [ERROR::ACCESS_DENIED]  if `msg.hwnd` belongs to another process.
/// *   [ERROR::MESSAGE_SYNC_ONLY]  if `msg.message` is a system message to be handled syncronously.<br>
///     Common for messages w/ pointers.  This occurs even if `hwnd` belongs to the current thread.
///
/// ### Example
/// ```
/// use hwnd::*;
///
/// fn main() {
/// #   post_quit_message(0); // don't hang test
///     // ...
///
///     loop {
///         let mut msg = Msg::zeroed();
///         get_message_a(&mut msg, HWnd::NULL, 0, 0).unwrap();
///
///         if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///
///         translate_message(&msg);
///         let _ = unsafe { dispatch_message_a(&msg) };
///     }
/// }
/// ```
pub unsafe fn dispatch_message_a(msg: &impl AsRef<Msg>) -> Result<LRESULT, Error> {
    fn_context!(dispatch_message_a => DispatchMessageA);
    let msg = msg.as_ref().as_ref();
    clear_last_error();
    let lr = unsafe { DispatchMessageA(msg) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew)\]
/// DispatchMessageW
///
/// Dispatches a message to a window procedure. It is typically used to dispatch a message retrieved by the [get_message](get_message_w) function.
///
/// ### Safety
/// *   `msg.wparam` and `msg.lparam` may need to be valid pointers, depending on the value of `msg.message` and the wndproc of `msg.hwnd`.
/// *   `msg.hwnd` should perhaps be a unicode window?
///
/// ### Returns
/// The return value specifies the value returned by the window procedure.
/// Although its meaning depends on the message being dispatched, the return value generally is ignored.
///
/// ### Errors
/// These are typically ignored!
/// *   [ERROR::INVALID_WINDOW_HANDLE]  if `msg.hwnd` was invalid, **or if the wndproc destroyed the window.**<br>
///     Common for WM::SYSCOMMAND (Alt+F4), WM::NCLBUTTONDOWN (clicked X), etc.
/// *   [ERROR::ACCESS_DENIED]  if `msg.hwnd` belongs to another process.
/// *   [ERROR::MESSAGE_SYNC_ONLY]  if `msg.message` is a system message to be handled syncronously.<br>
///     Common for messages w/ pointers.  This occurs even if `hwnd` belongs to the current thread.
///
/// ### Example
/// ```
/// use hwnd::*;
///
/// fn main() {
/// #   post_quit_message(0); // don't hang test
///     // ...
///
///     loop {
///         let mut msg = Msg::zeroed();
///         get_message_w(&mut msg, HWnd::NULL, 0, 0).unwrap();
///
///         if msg.message == WM::QUIT { std::process::exit(msg.wparam as _) }
///
///         translate_message(&msg);
///         let _ = unsafe { dispatch_message_w(&msg) };
///     }
/// }
/// ```
pub unsafe fn dispatch_message_w(msg: &impl AsRef<Msg>) -> Result<LRESULT, Error> {
    fn_context!(dispatch_message_w => DispatchMessageW);
    let msg = msg.as_ref().as_ref();
    clear_last_error();
    let lr = unsafe { DispatchMessageW(msg) };
    if lr == 0 { fn_error_gle_nz!()? }
    Ok(lr)
}

#[test] fn cross_thread_dispatch() {
    // perhaps despite expectations, this succeeds!
    use std::ptr::*;
    let hwnd = unsafe { create_window_a(abistr::cstr!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()).unwrap() };
    let _ : LRESULT = std::thread::spawn(move || unsafe {
        dispatch_message_w(&Msg {
            hwnd,
            message: WM::NULL,
            ..Default::default()
        })
    }).join().unwrap().unwrap();
}

#[test] fn cross_process_dispatch() {
    assert_eq!(
        ERROR::ACCESS_DENIED,
        unsafe{dispatch_message_w(&Msg {
            hwnd: get_desktop_window(),
            message: WM::NULL,
            ..Default::default()
        })}.unwrap_err()
    );
}
