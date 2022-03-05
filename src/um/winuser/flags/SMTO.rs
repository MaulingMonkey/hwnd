//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
//! SMTO_\* window style flags for [send_message_timeout](send_message_timeout_w)

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
/// SMTO_\* window style flags for [send_message_timeout](send_message_timeout_w)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct SendMessageTimeOutFlags(u32);
impl_ops_for_flag!(SendMessageTimeOutFlags);

impl From<SendMessageTimeOutFlags> for u32 { fn from(cmd: SendMessageTimeOutFlags) -> Self { cmd.0 } }
impl From<u32> for SendMessageTimeOutFlags { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl_debug_for_flags! {
    SendMessageTimeOutFlags => {
        SMTO::ABORTIFHUNG,
        SMTO::BLOCK,
        SMTO::NORMAL,
        SMTO::NOTIMEOUTIFNOTHUNG,
        SMTO::ERRORONEXIT,
    }
}



pub const ABORTIFHUNG           : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_ABORTIFHUNG);
pub const BLOCK                 : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_BLOCK);
pub const NORMAL                : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_NORMAL);
pub const NOTIMEOUTIFNOTHUNG    : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_NOTIMEOUTIFNOTHUNG);
pub const ERRORONEXIT           : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_ERRORONEXIT);
