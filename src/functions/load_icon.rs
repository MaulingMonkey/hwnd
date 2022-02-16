use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora)\]
/// IDC_\* flags for [load_cursor_w]
#[allow(non_snake_case)]
pub mod IDC {
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
}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora)\]
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
/// let idc_arrow = load_cursor_a(None, IDC::ARROW).unwrap();
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
/// let idc_arrow = load_cursor_w(None, IDC::ARROW).unwrap();
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
