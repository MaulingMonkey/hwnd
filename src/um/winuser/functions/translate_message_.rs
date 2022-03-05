use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-translatemessage)\]
/// TranslateMessage
///
/// Translates [WM::KEYDOWN], [WM::KEYUP], [WM::SYSKEYDOWN], and [WM::SYSKEYUP] into character events like [WM::CHAR], [WM::DEADCHAR], [WM::SYSCHAR], and [WM::SYSDEADCHAR].
/// The character messages are posted to the calling thread's message queue, to be read the next time the thread calls the [get_message](get_message_w) or [peek_message](peek_message_w) functions.
///
// TODO: document "remarks" and cases where you wouldn't call this, such as when calling [translate_accelerator]
/// ### Returns
/// *   `true`  if this translated one of the listed key event types.
/// *   `false` if this did nothing.
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
pub fn translate_message(msg: &impl AsRef<Msg>) -> bool {
    fn_context!(translate_message => TranslateMessage);
    unsafe { TranslateMessage(msg.as_ref().as_ref()) != 0 }
}
