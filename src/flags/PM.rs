//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)\]
//! PM_\* flags for [peek_message](peek_message_w)

#![allow(non_snake_case)]

use crate::*;
use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagew)\]
/// PM_\* flags for [peek_message](peek_message_w)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Zeroable)] #[repr(transparent)] pub struct PeekMessageFlags(u32);

impl From<PeekMessageFlags> for u32 { fn from(cmd: PeekMessageFlags) -> Self { cmd.0 } }
impl From<u32> for PeekMessageFlags { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl Debug for PeekMessageFlags {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "PM::{{")?;

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
            PM::NOREMOVE,
            PM::REMOVE,
            PM::NOYIELD,
            PM::QS_INPUT,
            PM::QS_PAINT,
            PM::QS_POSTMESSAGE,
            PM::QS_SENDMESSAGE,
        };

        let _ = (remaining, prev);

        write!(fmt, "}}")
    }
}



pub const NOREMOVE          : PeekMessageFlags = PeekMessageFlags(PM_NOREMOVE);
pub const REMOVE            : PeekMessageFlags = PeekMessageFlags(PM_REMOVE);
pub const NOYIELD           : PeekMessageFlags = PeekMessageFlags(PM_NOYIELD);
pub const QS_INPUT          : PeekMessageFlags = PeekMessageFlags(PM_QS_INPUT);
pub const QS_PAINT          : PeekMessageFlags = PeekMessageFlags(PM_QS_PAINT);
pub const QS_POSTMESSAGE    : PeekMessageFlags = PeekMessageFlags(PM_QS_POSTMESSAGE);
pub const QS_SENDMESSAGE    : PeekMessageFlags = PeekMessageFlags(PM_QS_SENDMESSAGE);
