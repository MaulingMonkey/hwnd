use crate::*;
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursora)\]
/// LoadCursorA
///
/// Loads a cursor, animated cursor, or bitmap.
///
/// ### Errors
/// *   [ERROR::RESOURCE_DATA_NOT_FOUND]        if `cursor_name` cannot be found for `Some(hinstance)`
/// *   [ERROR::RESOURCE_NAME_NOT_FOUND]        if `cursor_name` cannot be found for the system (e.g. `hinstance` is `None`)
/// *   ~~[ERROR::RESOURCE_TYPE_NOT_FOUND]~~    never? (returned instead of [ERROR::RESOURCE_NAME_NOT_FOUND] by [load_icon_a])
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winresult::*;
/// let idc_arrow = load_cursor_a(None, IDC::ARROW).unwrap();
///
/// let exe = get_module_handle_entry_exe().unwrap();
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_a(None, IDI::WINLOGO).unwrap_err(), "icon-only atom to load_cursor");
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_a(None, 42).unwrap_err(), "(None, 42)");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_a(exe,  42).unwrap_err(), "(exe,  42)");
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_a(None, cstr!("nonexistant")).unwrap_err(), "(None, \"nonexistant\")");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_a(exe,  cstr!("nonexistant")).unwrap_err(), "(exe,  \"nonexistant\")");
/// ```
///
/// ### See Also
/// *   [load_cursor_w] &mdash; wide equivalent
/// *   [load_icon_a]   &mdash; icon equivalent
pub fn load_cursor_a<'h, 't>(hinstance: impl Into<HInstance<'h>>, cursor_name: impl Into<NameAtomOrZero<'t, u8>>) -> Result<HCursor<'h>, Error> {
    fn_context!(load_cursor_a => LoadCursorA);
    let hcursor = unsafe { LoadCursorA(hinstance.into().into(), cursor_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hcursor.is_null())?;
    Ok(unsafe { HCursor::from_unchecked(hcursor) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
/// LoadCursorW
///
/// Loads a cursor, animated cursor, or bitmap.
///
/// ### Errors
/// *   [ERROR::RESOURCE_DATA_NOT_FOUND]        if `cursor_name` cannot be found for `Some(hinstance)`
/// *   [ERROR::RESOURCE_NAME_NOT_FOUND]        if `cursor_name` cannot be found for the system (e.g. `hinstance` is `None`)
/// *   ~~[ERROR::RESOURCE_TYPE_NOT_FOUND]~~    never? (returned instead of [ERROR::RESOURCE_NAME_NOT_FOUND] by [load_icon_w])
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winresult::*;
/// let idc_arrow = load_cursor_w(None, IDC::ARROW).unwrap();
///
/// let exe = get_module_handle_entry_exe().unwrap();
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, IDI::WINLOGO).unwrap_err(), "icon-only atom to load_cursor");
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, 42).unwrap_err(), "(None, 42)");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_w(exe,  42).unwrap_err(), "(exe,  42)");
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, cstr16!("nonexistant")).unwrap_err(), "(None, \"nonexistant\")");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_w(exe,  cstr16!("nonexistant")).unwrap_err(), "(exe,  \"nonexistant\")");
/// ```
///
/// ### See Also
/// *   [load_cursor_a] &mdash; narrow equivalent
/// *   [load_icon_w]   &mdash; icon equivalent
pub fn load_cursor_w<'h, 't>(hinstance: impl Into<HInstance<'h>>, cursor_name: impl Into<NameAtomOrZero<'t, u16>>) -> Result<HCursor<'h>, Error> {
    fn_context!(load_cursor_w => LoadCursorW);
    let hcursor = unsafe { LoadCursorW(hinstance.into().into(), cursor_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hcursor.is_null())?;
    Ok(unsafe { HCursor::from_unchecked(hcursor) })
}
