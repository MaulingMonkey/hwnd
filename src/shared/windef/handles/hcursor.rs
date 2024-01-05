use crate::*;
use bytemuck::*;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadcursorw)\]
/// HCURSOR
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default)]
pub struct HCursor<'a>(usize, PhantomData<&'a ()>);

impl HCursor<'_> {
    /// ### Safety
    /// *   `hcursor` must currently be valid or null
    /// *   `hcursor` must outlive `'_`
    pub unsafe fn from_unchecked(hcursor: HCURSOR) -> Self { Self(hcursor as _, PhantomData) }
}

unsafe impl Zeroable for HCursor<'_> {}

impl From<HCursor<'_>> for HCURSOR { fn from(c: HCursor) -> Self { c.0 as _ } }
impl From<()> for HCursor<'_> { fn from(_: ()) -> Self { Self(0, PhantomData) } }

impl Debug for HCursor<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HCursor(0x{:X})", self.0) } }
