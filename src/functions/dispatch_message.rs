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
