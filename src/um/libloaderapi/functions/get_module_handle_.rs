use crate::*;

use abistr::*;

use winapi::um::libloaderapi::*;

use std::ptr::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)\]
/// GetModuleHandleW\(nullptr\)
///
/// Retrieves a module handle for the entry executable.
///
/// ### Errors
/// *   Never?  Or perhaps on ReactOS / WINE / UWP?
///
/// ### Example
/// ```rust
/// # use hwnd::*;
/// let exe = get_module_handle_entry_exe().unwrap();
/// ```
pub fn get_module_handle_entry_exe() -> Result<HModule<'static>, Error> {
    fn_context!(get_module_handle_entry_exe => GetModuleHandleW);
    let hmodule = unsafe { GetModuleHandleW(null_mut()) };
    fn_succeeded!(!hmodule.is_null())?;
    unsafe { Ok(HModule::from_unchecked(hmodule)) }
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)\]
/// GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_PIN, ...)
///
/// Retrieves a module handle for the specified module.
/// The module must have been loaded by the calling process.
/// Uses `GET_MODULE_HANDLE_EX_FLAG_PIN`, so the resulting HMODULE will never be unloaded.
///
/// ### Errors
/// *   [ERROR::INVALID_PARAMETER]  If `module_name` contains interior nulls
/// *   [ERROR::MOD_NOT_FOUND]      If `module_name` is not loaded
///
/// ### Example
/// ```rust
/// # use abistr::*;
/// # use hwnd::*;
/// # use winresult::*;
/// let ntdll = get_module_handle_ex_w_pin(utf16ish!("ntdll")).unwrap();
/// assert_eq!(ERROR::MOD_NOT_FOUND, get_module_handle_ex_w_pin(utf16ish!("not_loaded")).unwrap_err());
/// ```
pub fn get_module_handle_ex_w_pin(module_name: impl TryIntoAsCStr<encoding::Utf16ish>) -> Result<HModule<'static>, Error> {
    fn_context!(get_module_handle_ex_w_pin => GetModuleHandleW);
    let module_name = module_name.try_into().map_err(|_| fn_param_error!(module_name, ERROR::INVALID_PARAMETER))?;
    let mut hmodule = null_mut();
    fn_succeeded!(unsafe { GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_PIN, module_name.as_cstr(), &mut hmodule) })?;
    fn_succeeded!(!hmodule.is_null())?;
    unsafe { Ok(HModule::from_unchecked(hmodule)) }
}
