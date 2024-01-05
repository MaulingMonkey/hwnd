//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)\]
//! InSendMessageEx return value flags

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex)\]
/// InSendMessageEx return value flags
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct InSendMessageExFlags(u32);
impl_ops_for_flag!(InSendMessageExFlags);

impl InSendMessageExFlags {
    // Per <https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-insendmessageex#remarks>
    pub fn is_sender_blocked(self) -> bool { self.0 & (ISMEX::REPLIED.0 | ISMEX::SEND.0) == ISMEX::SEND.0 }
}

impl From<InSendMessageExFlags> for u32 { fn from(cmd: InSendMessageExFlags) -> Self { cmd.0 } }
impl From<u32> for InSendMessageExFlags { fn from(cmd: u32                 ) -> Self { Self(cmd) } }

impl_debug_for_flags! {
    InSendMessageExFlags => {
        ISMEX::NOSEND,
        ISMEX::CALLBACK,
        ISMEX::NOTIFY,
        ISMEX::REPLIED,
        ISMEX::SEND,
    }
}



pub const NOSEND    : InSendMessageExFlags = InSendMessageExFlags(ISMEX_NOSEND);

/// The message was sent using the [send_message_callback](crate::send_message_callback_w) function. The thread that sent the message is not blocked.
pub const CALLBACK  : InSendMessageExFlags = InSendMessageExFlags(ISMEX_CALLBACK);

/// The message was sent using the [send_notify_message](crate::send_notify_message_w) function. The thread that sent the message is not blocked.
pub const NOTIFY    : InSendMessageExFlags = InSendMessageExFlags(ISMEX_NOTIFY);

/// The window procedure has processed the message. The thread that sent the message is no longer blocked.
pub const REPLIED   : InSendMessageExFlags = InSendMessageExFlags(ISMEX_REPLIED);

/// The message was sent using the [send_message](crate::send_message_w) or [send_message_timeout](crate::send_message_timeout_w) function.
/// If [ISMEX::REPLIED] is not set, the thread that sent the message is blocked.
pub const SEND      : InSendMessageExFlags = InSendMessageExFlags(ISMEX_SEND);
