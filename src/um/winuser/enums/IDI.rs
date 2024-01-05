//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadicona)\]
//! IDI_\* flags for [load_icon_w]
#![allow(non_snake_case)]

use crate::*;



// XXX: is AtomNonZero sufficient typing?
pub const APPLICATION   : AtomNonZero = AtomNonZero::from_constant(32512);
pub const HAND          : AtomNonZero = AtomNonZero::from_constant(32513);
pub const QUESTION      : AtomNonZero = AtomNonZero::from_constant(32514);
pub const EXCLAMATION   : AtomNonZero = AtomNonZero::from_constant(32515);
pub const ASTERISK      : AtomNonZero = AtomNonZero::from_constant(32516);
pub const WINLOGO       : AtomNonZero = AtomNonZero::from_constant(32517);       // WINVER >= 0x0400
pub const SHIELD        : AtomNonZero = AtomNonZero::from_constant(32518);       // WINVER >= 0x0600

pub const WARNING       : AtomNonZero = self::EXCLAMATION; // WINVER >= 0x0400
pub const ERROR         : AtomNonZero = self::HAND;        // WINVER >= 0x0400
pub const INFORMATION   : AtomNonZero = self::ASTERISK;    // WINVER >= 0x0400
