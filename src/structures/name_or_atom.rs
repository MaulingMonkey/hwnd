use abistr::Unit;
use bytemuck::*;
use std::marker::PhantomData;
use std::os::raw::c_char;



#[derive(Clone, Copy, Default)]
pub struct NameOrAtom<'a, U: Unit>(usize, PhantomData<Option<abistr::CStrNonNull<'a, U>>>);

unsafe impl<'a, U: Unit> Zeroable for NameOrAtom<'a, U> {} // Option<...>

impl<'a, U: Unit> NameOrAtom<'a, U> {
    pub fn to_atom(&self) -> Option<u16> {
        match u16::try_from(self.0) {
            Ok(0) => None, // not a valid atom
            Ok(n) => Some(n),
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

impl NameOrAtom<'_,  u8> { pub fn as_atom_or_cstr_ptr(&self) -> *const c_char { self.0 as _ } }
impl NameOrAtom<'_, u16> { pub fn as_atom_or_cstr_ptr(&self) -> *const u16    { self.0 as _ } }

impl<'a, U: Unit> From<u16                       > for NameOrAtom<'a, U> { fn from(atom:                     u16) -> Self { Self(atom as _, PhantomData) } }
impl<'a, U: Unit> From<abistr::CStrNonNull<'a, U>> for NameOrAtom<'a, U> { fn from(s: abistr::CStrNonNull<'a, U>) -> Self { Self(s.as_ptr() as _, PhantomData) } }



#[test] fn layout() {
    use std::mem::*;

    assert_eq!(align_of::<NameOrAtom<'_,  u8>>(), align_of::<*const  u8>());
    assert_eq!(size_of ::<NameOrAtom<'_,  u8>>(), size_of ::<*const  u8>());

    assert_eq!(align_of::<NameOrAtom<'_, u16>>(), align_of::<*const u16>());
    assert_eq!(size_of ::<NameOrAtom<'_, u16>>(), size_of ::<*const u16>());
}
