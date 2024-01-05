use crate::ISMEX::InSendMessageExFlags;
use winapi::um::winuser::*;
use std::ptr::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessage)\]
/// InSendMessage
///
/// Determines whether the current window procedure is processing a message that was sent from another thread (in the same process or a different process) by a call to [send_message](crate::send_message_w).
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// assert!(!in_send_message());
/// ```
pub fn in_send_message() -> bool {
    fn_context!(in_send_message => InSendMessage);
    unsafe { InSendMessage() != 0 }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)\]
/// InSendMessageEx
///
/// Determines whether the current window procedure is processing a message that was sent from another thread (in the same process or a different process).
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// assert_eq!(ISMEX::NOSEND, in_send_message_ex(None));
/// ```
pub fn in_send_message_ex(_reserved: impl Into<Option<std::convert::Infallible>>) -> InSendMessageExFlags {
    fn_context!(in_send_message_ex => InSendMessageEx);
    InSendMessageExFlags::from(unsafe { InSendMessageEx(null_mut()) })
}
