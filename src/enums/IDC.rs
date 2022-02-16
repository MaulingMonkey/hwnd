//! \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora)\]
//! IDC_\* flags for [load_cursor_w]
#![allow(non_snake_case)]

use crate::*;



// XXX: is Atom sufficient typing?
pub const ARROW         : Atom = Atom(32512);
pub const IBEAM         : Atom = Atom(32513);
pub const WAIT          : Atom = Atom(32514);
pub const CROSS         : Atom = Atom(32515);
pub const UPARROW       : Atom = Atom(32516);
pub const SIZE          : Atom = Atom(32640); // OBSOLETE: use IDC::SIZEALL
pub const ICON          : Atom = Atom(32641); // OBSOLETE: use IDC::ARROW
pub const SIZENWSE      : Atom = Atom(32642);
pub const SIZENESW      : Atom = Atom(32643);
pub const SIZEWE        : Atom = Atom(32644);
pub const SIZENS        : Atom = Atom(32645);
pub const SIZEALL       : Atom = Atom(32646);
pub const NO            : Atom = Atom(32648);
pub const HAND          : Atom = Atom(32649);
pub const APPSTARTING   : Atom = Atom(32650);
pub const HELP          : Atom = Atom(32651);
pub const PIN           : Atom = Atom(32671);
pub const PERSON        : Atom = Atom(32672);
