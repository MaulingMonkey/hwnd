use bytemuck::*;
use std::fmt::{self, Debug, Formatter};
use std::num::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Default, Pod, Zeroable)]
#[repr(transparent)]
pub struct Atom(pub(crate) u16);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM (nonzero)
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AtomNonZero(pub(crate) NonZeroU16);

impl Atom {
    pub const fn new(a: u16) -> Self { Self(a) }
    pub const fn to_u16(self) -> u16 { self.0 }
}

impl AtomNonZero {
    #[doc(hidden)] pub const fn from_constant(atom: u16) -> Self { assert!(atom != 0); Self(unsafe { NonZeroU16::new_unchecked(atom) }) }
    pub const fn new(atom: u16) -> Option<Self> { match NonZeroU16::new(atom) { Some(a) => Some(Self(a)), None => None } }
    pub const fn to_u16(self) -> u16 { self.0.get() }
}



impl From<u16           > for Atom          { fn from(a: u16            ) -> Self { Self(a) } }
impl From<NonZeroU16    > for Atom          { fn from(a: NonZeroU16     ) -> Self { Self(a.get()) } }
impl From<NonZeroU16    > for AtomNonZero   { fn from(a: NonZeroU16     ) -> Self { Self(a) } }
impl From<Atom          > for u16           { fn from(a: Atom           ) -> Self { a.to_u16() } }
impl From<AtomNonZero   > for u16           { fn from(a: AtomNonZero    ) -> Self { a.to_u16() } }
impl From<AtomNonZero   > for NonZeroU16    { fn from(a: AtomNonZero    ) -> Self { a.0 } }

impl Debug for Atom         { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "Atom(0x{:04x})", self.to_u16()) } }
impl Debug for AtomNonZero  { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { write!(fmt, "AtomNonZero(0x{:04x})", self.to_u16()) } }

convert!(AtomNonZero => unsafe { Atom });
