use crate::*;

use abistr::*;

use bytemuck::*;

use winapi::shared::minwindef::HMODULE;
use winapi::um::libloaderapi::*;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::num::*;
use std::ptr::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandlew)\]
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
pub fn get_module_handle_entry_exe() -> Result<HModuleNonNull<'static>, Error> {
    fn_context!(get_module_handle_entry_exe => GetModuleHandleW);
    let hmodule = unsafe { GetModuleHandleW(null_mut()) };
    Ok(HModuleNonNull(NonZeroUsize::new(hmodule as _).ok_or_else(|| fn_error_gle!())?, PhantomData))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexa)\]
/// GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_PIN, ...)
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
/// # use winerr::*;
/// let ntdll = get_module_handle_ex_a_pin("ntdll").unwrap();
/// let ntdll = get_module_handle_ex_a_pin(cstr!("ntdll")).unwrap();
/// assert_eq!(ERROR::MOD_NOT_FOUND,        get_module_handle_ex_a_pin("not_loaded").unwrap_err());
/// assert_eq!(ERROR::INVALID_PARAMETER,    get_module_handle_ex_a_pin("nt\0dll").unwrap_err());
/// ```
pub fn get_module_handle_ex_a_pin(module_name: impl TryIntoAsCStr) -> Result<HModuleNonNull<'static>, Error> {
    fn_context!(get_module_handle_ex_a_pin => GetModuleHandleA);
    let module_name = module_name.try_into().map_err(|_| fn_param_error!(module_name, ERROR::INVALID_PARAMETER))?;
    let mut hmodule = null_mut();
    fn_succeeded!(unsafe { GetModuleHandleExA(GET_MODULE_HANDLE_EX_FLAG_PIN, module_name.as_opt_cstr(), &mut hmodule) })?;
    Ok(HModuleNonNull(NonZeroUsize::new(hmodule as _).ok_or_else(|| fn_error_gle!())?, PhantomData))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getmodulehandleexw)\]
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
/// # use winerr::*;
/// let ntdll = get_module_handle_ex_w_pin(cstr16!("ntdll")).unwrap();
/// assert_eq!(ERROR::MOD_NOT_FOUND, get_module_handle_ex_w_pin(cstr16!("not_loaded")).unwrap_err());
/// ```
pub fn get_module_handle_ex_w_pin(module_name: impl TryIntoAsCStr<u16>) -> Result<HModuleNonNull<'static>, Error> {
    fn_context!(get_module_handle_ex_w_pin => GetModuleHandleW);
    let module_name = module_name.try_into().map_err(|_| fn_param_error!(module_name, ERROR::INVALID_PARAMETER))?;
    let mut hmodule = null_mut();
    fn_succeeded!(unsafe { GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_PIN, module_name.as_opt_cstr(), &mut hmodule) })?;
    Ok(HModuleNonNull(NonZeroUsize::new(hmodule as _).ok_or_else(|| fn_error_gle!())?, PhantomData))
}



/// The use of nearly always 'static lifetimes on HModule is *arguably* overkill.
///
/// FreeLibrary is unsafe as heck:
/// *    If unloading a Rust DLL, *any* `'static` references to data from said DLL will dangle.
/// *    C++ code is similarly unlikely to properly tolerate unloading it's DLL (background threads might not be terminated etc.)
///
/// That said, it adds an element of self-documentation that I appreciate.
/// e.g. it helped coax me to write [`get_module_handle_ex_a_pin`] instead of `get_module_handle_a`
const _STATIC_LIFETIME_SPAM : () = ();



/// \[[tont](https://devblogs.microsoft.com/oldnewthing/20040614-00/?p=38903)\]
/// HINSTANCE / HMODULE
///
/// Represents a loaded DLL or EXE inside the current process.
/// May be `0` / `null`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default)]
#[repr(transparent)]
pub struct HModule<'a>(usize, PhantomData<&'a ()>);

/// \[[tont](https://devblogs.microsoft.com/oldnewthing/20040614-00/?p=38903)\]
/// HINSTANCE / HMODULE
///
/// Represents a loaded DLL or EXE inside the current process.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HModuleNonNull<'a>(NonZeroUsize, PhantomData<&'a ()>);

pub use HModule         as HInstance;
pub use HModuleNonNull  as HInstanceNonNull;

impl<'a> HModule<'a> {
    /// ### Safety
    /// *   `hmodule` must currently be valid
    /// *   `hmodule` must outlive `'a`
    pub unsafe fn from_unchecked(hmodule: HMODULE) -> Self { Self(hmodule as _, PhantomData) }
}

impl<'a> HModuleNonNull<'a> {
    /// ### Safety
    /// *   `hmodule` must currently be valid
    /// *   `hmodule` must outlive `'a`
    pub unsafe fn from_unchecked(hmodule: HMODULE) -> Option<Self> { Some(Self(NonZeroUsize::new(hmodule as _)?, PhantomData)) }

    pub fn get(self) -> HModule<'a> { HModule(self.0.get(), self.1) }
}

unsafe impl Zeroable for HModule<'_> {}

impl Debug for HModule        <'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HModule(0x{:X})",         self.0) } }
impl Debug for HModuleNonNull <'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HModuleNonNull(0x{:X})",  self.0) } }

impl<'a> From<       HModuleNonNull<'a> > for HModule<'a> { fn from(hi:        HModuleNonNull<'a> ) -> Self { Self(hi.0.get(), PhantomData) } }
impl<'a> From<Option<HModuleNonNull<'a>>> for HModule<'a> { fn from(hi: Option<HModuleNonNull<'a>>) -> Self { hi.map_or(Self(0, PhantomData), |hi| Self(hi.0.get(), hi.1)) } }

impl From<HModule       <'_>> for HMODULE { fn from(hi: HModule         ) -> Self { hi.0       as _ } }
impl From<HModuleNonNull<'_>> for HMODULE { fn from(hi: HModuleNonNull  ) -> Self { hi.0.get() as _ } }
