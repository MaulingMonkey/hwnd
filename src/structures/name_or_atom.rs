use crate::*;
use abistr::Unit;
use bytemuck::*;
use std::marker::PhantomData;
use std::num::*;
use std::os::raw::c_char;


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM, a C-style string, or 0/null.
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub struct NameAtomOrZero<'a, U: Unit>(usize, PhantomData<Option<abistr::CStrNonNull<'a, U>>>);

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/dataxchg/about-atom-tables)\]
/// ATOM, or a C-style string.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct NameOrAtom<'a, U: Unit>(NonZeroUsize, PhantomData<Option<abistr::CStrNonNull<'a, U>>>);

unsafe impl<'a, U: Unit> Zeroable for NameAtomOrZero<'a, U> {} // Option<...>

impl<'a, U: Unit> NameAtomOrZero<'a, U> {
    pub fn is_zero(&self) -> bool { self.0 == 0 }

    pub fn to_atom_nz(&self) -> Option<AtomNonZero> {
        match u16::try_from(self.0) {
            Ok(n) => AtomNonZero::new(n),
            Err(_) => None,
        }
    }

    pub fn to_abistr(&self) -> Option<abistr::CStrNonNull<'a, U>> {
        if u16::try_from(self.0).is_ok() {
            None
        } else {
            Some(unsafe { abistr::CStrNonNull::from_ptr_unchecked_unbounded(self.0 as _) })
        }
    }
}

impl<'a, U: Unit> NameOrAtom<'a, U> {
    pub fn to_atom_nz(&self) -> Option<AtomNonZero> {
        match NonZeroU16::try_from(self.0) {
            Ok(n) => Some(AtomNonZero(n)),
            Err(_) => None,
        }
    }

    pub fn to_abistr(&self) -> Option<abistr::CStrNonNull<'a, U>> {
        if NonZeroU16::try_from(self.0).is_ok() {
            None
        } else {
            Some(unsafe { abistr::CStrNonNull::from_ptr_unchecked_unbounded(self.0.get() as _) })
        }
    }
}

impl NameAtomOrZero<'_,  u8> { pub fn as_atom_or_cstr_ptr(&self) -> *const c_char { self.0 as _ } }
impl NameAtomOrZero<'_, u16> { pub fn as_atom_or_cstr_ptr(&self) -> *const u16    { self.0 as _ } }
impl NameOrAtom    <'_,  u8> { pub fn as_atom_or_cstr_ptr(&self) -> *const c_char { self.0.get() as _ } }
impl NameOrAtom    <'_, u16> { pub fn as_atom_or_cstr_ptr(&self) -> *const u16    { self.0.get() as _ } }

impl<'a, U: Unit> From<u16                          > for NameAtomOrZero<'a, U> { fn from(atom:                     u16) -> Self { Self(atom as _,          PhantomData) } }
impl<'a, U: Unit> From<Atom                         > for NameAtomOrZero<'a, U> { fn from(atom:                    Atom) -> Self { Self(atom.to_u16() as _, PhantomData) } }
impl<'a, U: Unit> From<AtomNonZero                  > for NameAtomOrZero<'a, U> { fn from(atom:             AtomNonZero) -> Self { Self(atom.to_u16() as _, PhantomData) } }
impl<'a, U: Unit> From<abistr::CStrPtr<'a, U>       > for NameAtomOrZero<'a, U> { fn from(s: abistr::CStrPtr    <'a, U>) -> Self { Self(s.as_ptr() as _,    PhantomData) } }
impl<'a, U: Unit> From<abistr::CStrNonNull<'a, U>   > for NameAtomOrZero<'a, U> { fn from(s: abistr::CStrNonNull<'a, U>) -> Self { Self(s.as_ptr() as _,    PhantomData) } }

impl<'a, U: Unit> From<AtomNonZero                  > for NameOrAtom<'a, U> { fn from(atom:             AtomNonZero) -> Self { Self(NonZeroUsize::new(atom.to_u16() as _).unwrap(), PhantomData) } }
impl<'a, U: Unit> From<abistr::CStrNonNull<'a, U>   > for NameOrAtom<'a, U> { fn from(s: abistr::CStrNonNull<'a, U>) -> Self { Self(NonZeroUsize::new(s.as_ptr() as _).unwrap(),    PhantomData) } }



#[test] fn layout() {
    use std::mem::*;

    assert_eq!(align_of::<NameAtomOrZero<'_,  u8>>(), align_of::<*const  u8>());
    assert_eq!(size_of ::<NameAtomOrZero<'_,  u8>>(), size_of ::<*const  u8>());

    assert_eq!(align_of::<NameAtomOrZero<'_, u16>>(), align_of::<*const u16>());
    assert_eq!(size_of ::<NameAtomOrZero<'_, u16>>(), size_of ::<*const u16>());

    assert_eq!(align_of::<NameOrAtom<'_,  u8>>(), align_of::<*const  u8>());
    assert_eq!(size_of ::<NameOrAtom<'_,  u8>>(), size_of ::<*const  u8>());

    assert_eq!(align_of::<NameOrAtom<'_, u16>>(), align_of::<*const u16>());
    assert_eq!(size_of ::<NameOrAtom<'_, u16>>(), size_of ::<*const u16>());
}
