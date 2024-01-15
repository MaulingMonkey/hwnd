use bytemuck::*;

use winapi::shared::minwindef::HMODULE;

use std::convert::Infallible;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;



/// The use of nearly always 'static lifetimes on HModule is *arguably* overkill.
///
/// FreeLibrary is unsafe as heck:
/// *    If unloading a Rust DLL, *any* `'static` references to data from said DLL will dangle.
/// *    C++ code is similarly unlikely to properly tolerate unloading it's DLL (background threads might not be terminated etc.)
///
/// That said, it adds an element of self-documentation that I appreciate.
/// e.g. it helped coax me to write [`get_module_handle_ex_w_pin`] instead of `get_module_handle_w`
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

pub use HModule as HInstance;

impl<'a> HModule<'a> {
    /// ### Safety
    /// *   `hmodule` must currently be valid
    /// *   `hmodule` must outlive `'a`
    pub unsafe fn from_unchecked(hmodule: HMODULE) -> Self { Self(hmodule as _, PhantomData) }
}

unsafe impl Zeroable for HModule<'_> {}

impl Debug for HModule<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HModule(0x{:X})", self.0) } }

impl<'a>    From<()>                    for HModule<'a> { fn from(_: ()                 ) -> Self { Self(0, PhantomData) } }
impl<'a>    From<Option<Infallible>>    for HModule<'a> { fn from(_: Option<Infallible> ) -> Self { Self(0, PhantomData) } }
impl        From<HModule<'_>>           for HMODULE     { fn from(hi: HModule           ) -> Self { hi.0 as _ } }
