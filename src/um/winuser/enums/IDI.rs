//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadicona)\]
//! IDI_\* flags for [load_icon_w]
#![allow(non_snake_case)]

use crate::*;



// XXX: is Atom sufficient typing?
pub const APPLICATION   : Atom = Atom(32512);
pub const HAND          : Atom = Atom(32513);
pub const QUESTION      : Atom = Atom(32514);
pub const EXCLAMATION   : Atom = Atom(32515);
pub const ASTERISK      : Atom = Atom(32516);
pub const WINLOGO       : Atom = Atom(32517);       // WINVER >= 0x0400
pub const SHIELD        : Atom = Atom(32518);       // WINVER >= 0x0600

pub const WARNING       : Atom = self::EXCLAMATION; // WINVER >= 0x0400
pub const ERROR         : Atom = self::HAND;        // WINVER >= 0x0400
pub const INFORMATION   : Atom = self::ASTERISK;    // WINVER >= 0x0400
