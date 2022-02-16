use bytemuck::*;

use winapi::shared::minwindef::HMODULE;

use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::num::*;



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
pub struct HModule<'a>(pub(crate) usize, pub(crate) PhantomData<&'a ()>);

/// \[[tont](https://devblogs.microsoft.com/oldnewthing/20040614-00/?p=38903)\]
/// HINSTANCE / HMODULE
///
/// Represents a loaded DLL or EXE inside the current process.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HModuleNonNull<'a>(pub(crate) NonZeroUsize, pub(crate) PhantomData<&'a ()>);

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
