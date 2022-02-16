use crate::*;

use bytemuck::*;
use winapi::um::winuser::*;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;



// XXX: is Atom sufficient?
pub const IDC_ARROW         : Atom = Atom(32512);
pub const IDC_IBEAM         : Atom = Atom(32513);
pub const IDC_WAIT          : Atom = Atom(32514);
pub const IDC_CROSS         : Atom = Atom(32515);
pub const IDC_UPARROW       : Atom = Atom(32516);
pub const IDC_SIZE          : Atom = Atom(32640);  /* OBSOLETE: use IDC_SIZEALL */
pub const IDC_ICON          : Atom = Atom(32641);  /* OBSOLETE: use IDC_ARROW */
pub const IDC_SIZENWSE      : Atom = Atom(32642);
pub const IDC_SIZENESW      : Atom = Atom(32643);
pub const IDC_SIZEWE        : Atom = Atom(32644);
pub const IDC_SIZENS        : Atom = Atom(32645);
pub const IDC_SIZEALL       : Atom = Atom(32646);
pub const IDC_NO            : Atom = Atom(32648); /*not in win3.1 */
pub const IDC_HAND          : Atom = Atom(32649);
pub const IDC_APPSTARTING   : Atom = Atom(32650); /*not in win3.1 */
pub const IDC_HELP          : Atom = Atom(32651);
pub const IDC_PIN           : Atom = Atom(32671);
pub const IDC_PERSON        : Atom = Atom(32672);



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
/// LoadCursorA
///
/// Loads an icon, cursor, animated cursor, or bitmap.
///
/// ### Errors
/// *   [ERROR::RESOURCE_NAME_NOT_FOUND]    if `cursor_name` cannot be found for `hinstance`
/// *   [ERROR::RESOURCE_DATA_NOT_FOUND]    if `cursor_name` cannot be found for the system, and is an integer resource (e.g. `hinstance` is `None`)
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winerr::*;
/// let idc_arrow = load_cursor_a(None, IDC_ARROW).unwrap();
///
/// let exe = get_module_handle_entry_exe().unwrap();
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_a(None, 42).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_a(exe,  42).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_a(None, cstr!("nonexistant")).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_a(exe,  cstr!("nonexistant")).unwrap_err());
/// ```
///
/// ### See Also
/// *   [load_cursor_w]
pub fn load_cursor_a<'h, 't>(hinstance: impl Into<HInstance<'h>>, cursor_name: impl Into<NameAtomOrZero<'t, u8>>) -> Result<HCursor<'h>, Error> {
    fn_context!(load_cursor_a => LoadCursorA);
    let hcursor = unsafe { LoadCursorA(hinstance.into().into(), cursor_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hcursor.is_null())?;
    Ok(unsafe { HCursor::from_unchecked(hcursor) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
/// LoadCursorW
///
/// Loads an icon, cursor, animated cursor, or bitmap.
///
/// ### Errors
/// *   [ERROR::RESOURCE_DATA_NOT_FOUND]    if `cursor_name` cannot be found for the system (e.g. `hinstance` is `None`)
/// *   [ERROR::RESOURCE_NAME_NOT_FOUND]    if `cursor_name` cannot be found for `hinstance`
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winerr::*;
/// let idc_arrow = load_cursor_w(None, IDC_ARROW).unwrap();
///
/// let exe = get_module_handle_entry_exe().unwrap();
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, 42).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_w(exe,  42).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, cstr16!("nonexistant")).unwrap_err());
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_w(exe,  cstr16!("nonexistant")).unwrap_err());
/// ```
///
/// ### See Also
/// *   [load_cursor_a]
pub fn load_cursor_w<'h, 't>(hinstance: impl Into<HInstance<'h>>, cursor_name: impl Into<NameAtomOrZero<'t, u16>>) -> Result<HCursor<'h>, Error> {
    fn_context!(load_cursor_w => LoadCursorW);
    let hcursor = unsafe { LoadCursorW(hinstance.into().into(), cursor_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hcursor.is_null())?;
    Ok(unsafe { HCursor::from_unchecked(hcursor) })
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
/// HCURSOR
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default)]
pub struct HCursor<'a>(usize, PhantomData<&'a ()>);

impl HCursor<'_> {
    /// ### Safety
    /// *   `hcursor` must currently be valid or null
    /// *   `hcursor` must outlive `'_`
    pub unsafe fn from_unchecked(hcursor: HCURSOR) -> Self { Self(hcursor as _, PhantomData) }
}

unsafe impl Zeroable for HCursor<'_> {}

impl From<HCursor<'_>> for HCURSOR { fn from(c: HCursor) -> Self { c.0 as _ } }
impl From<()> for HCursor<'_> { fn from(_: ()) -> Self { Self(0, PhantomData) } }

impl Debug for HCursor<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HCursor(0x{:X})", self.0) } }
