//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
//! SMTO_\* window style flags for [send_message_timeout](send_message_timeout_w)

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessagetimeoutw)\]
/// SMTO_\* window style flags for [send_message_timeout](send_message_timeout_w)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct SendMessageTimeOutFlags(u32);

impl From<SendMessageTimeOutFlags> for u32 { fn from(cmd: SendMessageTimeOutFlags) -> Self { cmd.0 } }
impl From<u32> for SendMessageTimeOutFlags { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl Debug for SendMessageTimeOutFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "SMTO::{{")?;

        let mut prev        = false;
        let mut remaining   = self.0;

        macro_rules! flags { ( $($ident:path),* $(,)? ) => {$({
            let mask : u32 = ($ident).0;
            if (remaining & mask) != 0 {
                if prev { write!(fmt, " | ")?; }
                write!(fmt, "{}", stringify!($ident))?;
                prev        = true;
                remaining   = remaining & !mask;
            }
        })*}}

        flags! {
            SMTO::ABORTIFHUNG,
            SMTO::BLOCK,
            SMTO::NORMAL,
            SMTO::NOTIMEOUTIFNOTHUNG,
            SMTO::ERRORONEXIT,
        };

        let _ = (remaining, prev);

        write!(fmt, "}}")
    }
}



pub const ABORTIFHUNG           : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_ABORTIFHUNG);
pub const BLOCK                 : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_BLOCK);
pub const NORMAL                : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_NORMAL);
pub const NOTIMEOUTIFNOTHUNG    : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_NOTIMEOUTIFNOTHUNG);
pub const ERRORONEXIT           : SendMessageTimeOutFlags = SendMessageTimeOutFlags(SMTO_ERRORONEXIT);
