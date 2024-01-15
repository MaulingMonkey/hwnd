use crate::*;
use abistr::{CStrNonNull, CStrPtr};
use abistr::encoding::{Encoding, Utf16ish};
use bytemuck::*;
use std::marker::PhantomData;
use std::num::*;



/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM, a C-style string, or 0/null.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct NameAtomOrZero<'a, E: Encoding>(usize, PhantomData<Option<CStrNonNull<'a, E>>>);

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM, or a C-style string.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct NameOrAtom<'a, E: Encoding>(NonZeroUsize, PhantomData<Option<CStrNonNull<'a, E>>>);

unsafe impl<'a, E: Encoding> Zeroable for NameAtomOrZero<'a, E> {} // Option<...>

impl<'a, E: Encoding> NameAtomOrZero<'a, E> {
    pub fn is_zero(&self) -> bool { self.0 == 0 }

    pub fn to_atom_nz(&self) -> Option<AtomNonZero> {
        match u16::try_from(self.0) {
            Ok(n) => AtomNonZero::new(n),
            Err(_) => None,
        }
    }

    pub fn to_abistr(&self) -> Option<CStrNonNull<'a, E>> {
        if u16::try_from(self.0).is_ok() {
            None
        } else {
            Some(unsafe { CStrNonNull::from_ptr_unchecked(self.0 as _) })
        }
    }
}

impl<'a, E: Encoding> NameOrAtom<'a, E> {
    pub fn to_atom_nz(&self) -> Option<AtomNonZero> {
        match NonZeroU16::try_from(self.0) {
            Ok(n) => Some(AtomNonZero(n)),
            Err(_) => None,
        }
    }

    pub fn to_abistr(&self) -> Option<CStrNonNull<'a, E>> {
        if NonZeroU16::try_from(self.0).is_ok() {
            None
        } else {
            Some(unsafe { CStrNonNull::from_ptr_unchecked(self.0.get() as _) })
        }
    }
}

impl NameAtomOrZero<'_, Utf16ish> { pub fn as_atom_or_cstr_ptr(&self) -> *const u16    { self.0 as _ } }
impl NameOrAtom    <'_, Utf16ish> { pub fn as_atom_or_cstr_ptr(&self) -> *const u16    { self.0.get() as _ } }

impl<'a, E: Encoding> From<u16                  > for NameAtomOrZero<'a, E> { fn from(atom:             u16) -> Self { Self(atom as _,          PhantomData) } }
impl<'a, E: Encoding> From<Atom                 > for NameAtomOrZero<'a, E> { fn from(atom:            Atom) -> Self { Self(atom.to_u16() as _, PhantomData) } }
impl<'a, E: Encoding> From<AtomNonZero          > for NameAtomOrZero<'a, E> { fn from(atom:     AtomNonZero) -> Self { Self(atom.to_u16() as _, PhantomData) } }
impl<'a, E: Encoding> From<CStrPtr<'a, E>       > for NameAtomOrZero<'a, E> { fn from(s: CStrPtr    <'a, E>) -> Self { Self(s.as_ptr() as _,    PhantomData) } }
impl<'a, E: Encoding> From<CStrNonNull<'a, E>   > for NameAtomOrZero<'a, E> { fn from(s: CStrNonNull<'a, E>) -> Self { Self(s.as_ptr() as _,    PhantomData) } }

impl<'a, E: Encoding> From<AtomNonZero          > for NameOrAtom<'a, E> { fn from(atom:     AtomNonZero) -> Self { Self(NonZeroUsize::new(atom.to_u16() as _).unwrap(), PhantomData) } }
impl<'a, E: Encoding> From<CStrNonNull<'a, E>   > for NameOrAtom<'a, E> { fn from(s: CStrNonNull<'a, E>) -> Self { Self(NonZeroUsize::new(s.as_ptr() as _).unwrap(),    PhantomData) } }



#[test] fn layout() {
    use std::mem::*;

    assert_eq!(align_of::<NameAtomOrZero<'_, Utf16ish>>(), align_of::<*const u16>());
    assert_eq!(size_of ::<NameAtomOrZero<'_, Utf16ish>>(), size_of ::<*const u16>());

    assert_eq!(align_of::<NameOrAtom<'_, Utf16ish>>(), align_of::<*const u16>());
    assert_eq!(size_of ::<NameOrAtom<'_, Utf16ish>>(), size_of ::<*const u16>());
}
