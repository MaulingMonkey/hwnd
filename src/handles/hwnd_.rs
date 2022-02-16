use bytemuck::*;

use winapi::shared::windef::HWND;

use std::convert::Infallible;
use std::fmt::{self, Debug, Formatter};



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/learnwin32/what-is-a-window-)\]
/// HWND
///
/// Represents a handle to a window.<br>
/// May or may not belong to the current thread.<br>
/// May or may not belong to the current *process*.<br>
/// May or may not have ever existed, or be valid.<br>
/// May be some kind of magic number or value, like HWND_PARENT.<br>
/// May be null.<br>
///
/// ### Safety
///
/// HWNDs are awkward and fragile.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
#[repr(transparent)]
pub struct HWnd(pub(crate) usize);

impl HWnd {
    pub const fn is_null(self) -> bool { self.0 == 0 }
}

impl Debug for HWnd { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HWnd(0x{:X})", self.0) } }

impl From<()>                   for HWnd { fn from(_: ()                 ) -> Self { Self(0) } }
impl From<Option<Infallible>>   for HWnd { fn from(_: Option<Infallible> ) -> Self { Self(0) } }
impl From<HWnd>                 for HWND { fn from(h: HWnd               ) -> Self { h.0 as _ } }
impl From<HWND>                 for HWnd { fn from(h: HWND               ) -> Self { Self(h as _) } }
