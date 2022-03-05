macro_rules! fn_context     { ( $cpp:path => $rust:path ) => {} }
macro_rules! fn_succeeded   { ( $expr:expr ) => { if false == abibool::b32::from($expr) { Err($crate::Error::new_gle()) } else { Ok(()) } }}
macro_rules! fn_err         { (               $error:expr ) => { Err($crate::Error(($error).into())) } }
macro_rules! fn_error       { (               $error:expr ) => {     $crate::Error(($error).into())  } }
macro_rules! fn_param_error { ( $_param:expr, $error:expr ) => {     $crate::Error(($error).into())  } }
macro_rules! fn_error_gle   { (                           ) => {     $crate::Error::new_gle()        } }
macro_rules! fn_error_gle_nz{ (                           ) => {     $crate::Error::new_gle_nz()     } }

macro_rules! mods {
    ( $( #[$attr:meta] )* inl      mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])*      mod $mod;                       #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* inl      mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])*      mod $mod { mods!{ $($body)* } } #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod;                                                                 mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod { mods!{ $($body)* } }                                           mods!{ $($tt)* } };
    () => {};
}

/// ### Usage
/// ```no_compile
/// convert!(Rect <=> unsafe { winapi::shared::windef::RECT, winapi::shared::windef::RECTL, winapi::shared::d3d9types::D3DRECT });
/// ```
macro_rules! convert {
    ( $( $self:ident <=> unsafe { $($path:path),+ $(,)? } ),* $(,)? ) => {
        $(
            impl AsRef<$self> for $self { fn as_ref(&    self) -> &    $self { self } }
            impl AsMut<$self> for $self { fn as_mut(&mut self) -> &mut $self { self } }

            $(
                impl From< $path> for $self { fn from(r: $path) -> Self { unsafe { std::mem::transmute(r) } } }
                impl From< $self> for $path { fn from(r: $self) -> Self { unsafe { std::mem::transmute(r) } } }
                impl AsRef<$path> for $self { fn as_ref(&    self) -> &    $path { unsafe { std::mem::transmute(self) } } }
                impl AsRef<$self> for $path { fn as_ref(&    self) -> &    $self { unsafe { std::mem::transmute(self) } } }
                impl AsMut<$path> for $self { fn as_mut(&mut self) -> &mut $path { unsafe { std::mem::transmute(self) } } }
                impl AsMut<$self> for $path { fn as_mut(&mut self) -> &mut $self { unsafe { std::mem::transmute(self) } } }
            )*
        )*

        #[test] fn layout() {
            use std::mem::*;
            $($(
            assert_eq!(size_of ::<$self>(), size_of ::<$path>());
            assert_eq!(align_of::<$self>(), align_of::<$path>());
            )*)*
        }
    };
    ( $( $self:ident => unsafe { $($path:path),+ $(,)? } ),* $(,)? ) => {
        $(
            impl AsRef<$self> for $self { fn as_ref(&    self) -> &    $self { self } }
            impl AsMut<$self> for $self { fn as_mut(&mut self) -> &mut $self { self } }

            $(
                impl From<$self> for $path { fn from(r: $self) -> Self { unsafe { std::mem::transmute(r) } } }
                impl AsRef<$path> for $self { fn as_ref(&self) -> &$path { unsafe { std::mem::transmute(self) } } }
            )*
        )*

        #[test] fn layout() {
            use std::mem::*;
            $($(
            assert_eq!(size_of ::<$self>(), size_of ::<$path>(), "size_of {}", stringify!($path));
            assert_eq!(align_of::<$self>(), align_of::<$path>(), "align_of {}", stringify!($path));
            )*)*
        }
    };
    ( $( $self:ident<'_> => unsafe { $($path:path),+ $(,)? } ),* $(,)? ) => {
        $(
            //impl AsRef<$self<'_>> for $self<'_> { fn as_ref(&    self) -> &    $self { self } }
            //impl AsMut<$self<'_>> for $self<'_> { fn as_mut(&mut self) -> &mut $self { self } }

            $(
                impl From<$self<'_>> for $path { fn from(r: $self) -> Self { unsafe { std::mem::transmute(r) } } }
                impl AsRef<$path> for $self<'_> { fn as_ref(&self) -> &$path { unsafe { std::mem::transmute(self) } } }
            )*
        )*

        #[test] fn layout() {
            use std::mem::*;
            $($(
            assert_eq!(size_of ::<$self>(), size_of ::<$path>(), "size_of {}", stringify!($path));
            assert_eq!(align_of::<$self>(), align_of::<$path>(), "align_of {}", stringify!($path));
            )*)*
        }
    };
}

macro_rules! impl_ops_for_flag {( $flag:ty ) => {
    impl std::ops::Not              for $flag { type Output = $flag; fn not(self) -> Self { Self(!self.0) } }

    impl std::ops::BitOr            for $flag { type Output = $flag; fn bitor (self, other: Self) -> Self { Self(self.0 | other.0) } }
    impl std::ops::BitAnd           for $flag { type Output = $flag; fn bitand(self, other: Self) -> Self { Self(self.0 & other.0) } }
    impl std::ops::BitXor           for $flag { type Output = $flag; fn bitxor(self, other: Self) -> Self { Self(self.0 ^ other.0) } }

    impl std::ops::BitOrAssign      for $flag { fn bitor_assign (&mut self, other: Self) { self.0 |= other.0 } }
    impl std::ops::BitAndAssign     for $flag { fn bitand_assign(&mut self, other: Self) { self.0 &= other.0 } }
    impl std::ops::BitXorAssign     for $flag { fn bitxor_assign(&mut self, other: Self) { self.0 ^= other.0 } }

    impl $flag {
        /// `true` if all bits of `other` are set in `self`<br>
        /// `true` if `other` is `0`
        pub const fn has_all(self, other: Self) -> bool { self.0 & other.0 == other.0 }

        /// `true` if all bits of `other` are set in `self`<br>
        /// `false` if `other` is `0`
        pub const fn has_any(self, other: Self) -> bool { self.0 & other.0 != 0 }
    }
}}

macro_rules! impl_debug_for_enum {( $flag:ty => { $($path:path),* $(,)? } ) => {
    impl std::fmt::Debug for $flag {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            let s = match *self {
            $(  $path   => stringify!($path), )*
                _       => return write!(fmt, "{}", self.0),
            };
            fmt.write_str(s)
        }
    }
}}

macro_rules! impl_debug_for_flags {( $flag:ty => { $($path:path),* $(,)? } ) => {
    impl std::fmt::Debug for $flag {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            match *self {
            $(  $path   => return fmt.write_str(stringify!($path)), )*
                #[allow(unreachable_patterns)]
                Self(0) => return fmt.write_str("0"),
                _       => {},
            };
            let mut remaining = *self;
            $(
                if $path.0 != 0 && remaining.has_all($path) {
                    if *self != remaining { fmt.write_str(" | ")?; }
                    fmt.write_str(stringify!($path))?;
                    remaining &=! $path;
                }
            )*

            if remaining.0 != 0 {
                if *self != remaining { fmt.write_str(" | ")?; }
                write!(fmt, "0x{:X}", self.0)?;
            }

            Ok(())
        }
    }
}}
