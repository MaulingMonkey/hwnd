use crate::*;
use bytemuck::*;
use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadiconw)\]
/// HICON
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default)]
pub struct HIcon<'a>(usize, PhantomData<&'a ()>);

impl HIcon<'_> {
    /// ### Safety
    /// *   `hicon` must currently be valid or null
    /// *   `hicon` must outlive `'_`
    pub unsafe fn from_unchecked(hicon: HICON) -> Self { Self(hicon as _, PhantomData) }
}

unsafe impl Zeroable for HIcon<'_> {}

impl From<HIcon<'_>> for HICON { fn from(c: HIcon) -> Self { c.0 as _ } }
impl From<()> for HIcon<'_> { fn from(_: ()) -> Self { Self(0, PhantomData) } }

impl Debug for HIcon<'_> { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "HIcon(0x{:X})", self.0) } }
