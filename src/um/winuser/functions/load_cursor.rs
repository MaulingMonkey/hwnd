use crate::*;
use abistr::encoding::Utf16ish;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
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
/// assert_eq!(ERROR::RESOURCE_NAME_NOT_FOUND, load_cursor_w(None, utf16ish!("nonexistant")).unwrap_err(), "(None, \"nonexistant\")");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_cursor_w(exe,  utf16ish!("nonexistant")).unwrap_err(), "(exe,  \"nonexistant\")");
/// ```
///
/// ### See Also
/// *   [load_icon_w]   &mdash; icon equivalent
pub fn load_cursor_w<'h, 't>(hinstance: impl Into<HInstance<'h>>, cursor_name: impl Into<NameAtomOrZero<'t, Utf16ish>>) -> Result<HCursor<'h>, Error> {
    fn_context!(load_cursor_w => LoadCursorW);
    let hcursor = unsafe { LoadCursorW(hinstance.into().into(), cursor_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hcursor.is_null())?;
    Ok(unsafe { HCursor::from_unchecked(hcursor) })
}
