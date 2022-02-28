use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-replymessage)\]
/// ReplyMessage
///
/// Replies to a message sent from another thread by the [send_message](send_message_w) function.
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// // Not in a wndproc, so this will always fail:
/// reply_message(0).unwrap_err();
// TODO: non-failing example
/// ```
pub fn reply_message(lresult: LRESULT) -> Result<(), ReplyMessageError> {
    fn_context!(reply_message => ReplyMessage);
    match unsafe { ReplyMessage(lresult) } {
        0       => Err(ReplyMessageError(())),
        1 | _   => Ok(()),
    }
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-replymessage)\]
/// [reply_message] failed (wasn't handling a message sent by [send_message](send_message_w) or similar.)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct ReplyMessageError(());

impl std::fmt::Display for ReplyMessageError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "ReplyMessage failed: wasn't handling a message sent by SendMessage et al.")
    }
}
