use crate::*;
use abistr::encoding::Utf16ish;
use winapi::um::winuser::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)\]
/// LoadIconW
///
/// Loads an icon, animated icon, or bitmap.
///
/// ### Errors
/// *   [ERROR::RESOURCE_DATA_NOT_FOUND]        if `icon_name` cannot be found for `Some(hinstance)`
/// *   ~~[ERROR::RESOURCE_NAME_NOT_FOUND]~~    never? (returned by [load_cursor_w])
/// *   [ERROR::RESOURCE_TYPE_NOT_FOUND]        if `icon_name` cannot be found for the system (e.g. `hinstance` is `None`)
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winresult::*;
/// let idi_error = load_icon_w(None, IDI::ERROR).unwrap();
///
/// let exe = get_module_handle_entry_exe().unwrap();
/// assert_eq!(ERROR::RESOURCE_TYPE_NOT_FOUND, load_icon_w(None, IDC::SIZE).unwrap_err(), "cursor-only atom to load_icon");
/// assert_eq!(ERROR::RESOURCE_TYPE_NOT_FOUND, load_icon_w(None, 42).unwrap_err(), "(None, 42)");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_icon_w(exe,  42).unwrap_err(), "(exe,  42)");
/// assert_eq!(ERROR::RESOURCE_TYPE_NOT_FOUND, load_icon_w(None, utf16ish!("nonexistant")).unwrap_err(), "(None, \"nonexistant\")");
/// assert_eq!(ERROR::RESOURCE_DATA_NOT_FOUND, load_icon_w(exe,  utf16ish!("nonexistant")).unwrap_err(), "(exe,  \"nonexistant\")");
/// ```
///
/// ### See Also
/// *   [load_cursor_w] &mdash; cursor equivalent
pub fn load_icon_w<'h, 't>(hinstance: impl Into<HInstance<'h>>, icon_name: impl Into<NameAtomOrZero<'t, Utf16ish>>) -> Result<HIcon<'h>, Error> {
    fn_context!(load_icon_w => LoadIconW);
    let hicon = unsafe { LoadIconW(hinstance.into().into(), icon_name.into().as_atom_or_cstr_ptr()) };
    fn_succeeded!(!hicon.is_null())?;
    Ok(unsafe { HIcon::from_unchecked(hicon) })
}
