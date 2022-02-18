use crate::*;
use winapi::um::winuser::*;



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
/// # use winresult::*;
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
/// # use winresult::*;
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
