//! \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora)\]
//! IDC_\* flags for [load_cursor_w]
#![allow(non_snake_case)]

use crate::*;



// XXX: is AtomNonZero sufficient typing?
pub const ARROW         : AtomNonZero = AtomNonZero::from_constant(32512);
pub const IBEAM         : AtomNonZero = AtomNonZero::from_constant(32513);
pub const WAIT          : AtomNonZero = AtomNonZero::from_constant(32514);
pub const CROSS         : AtomNonZero = AtomNonZero::from_constant(32515);
pub const UPARROW       : AtomNonZero = AtomNonZero::from_constant(32516);
pub const SIZE          : AtomNonZero = AtomNonZero::from_constant(32640); // OBSOLETE: use IDC::SIZEALL
pub const ICON          : AtomNonZero = AtomNonZero::from_constant(32641); // OBSOLETE: use IDC::ARROW
pub const SIZENWSE      : AtomNonZero = AtomNonZero::from_constant(32642);
pub const SIZENESW      : AtomNonZero = AtomNonZero::from_constant(32643);
pub const SIZEWE        : AtomNonZero = AtomNonZero::from_constant(32644);
pub const SIZENS        : AtomNonZero = AtomNonZero::from_constant(32645);
pub const SIZEALL       : AtomNonZero = AtomNonZero::from_constant(32646);
pub const NO            : AtomNonZero = AtomNonZero::from_constant(32648);
pub const HAND          : AtomNonZero = AtomNonZero::from_constant(32649);
pub const APPSTARTING   : AtomNonZero = AtomNonZero::from_constant(32650);
pub const HELP          : AtomNonZero = AtomNonZero::from_constant(32651);
pub const PIN           : AtomNonZero = AtomNonZero::from_constant(32671);
pub const PERSON        : AtomNonZero = AtomNonZero::from_constant(32672);
