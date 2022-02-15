macro_rules! fn_context     { ( $cpp:path => $rust:path ) => {} }
macro_rules! fn_succeeded   { ( $expr:expr ) => { if false == abibool::b32::from($expr) { Err($crate::Error::new_gle()) } else { Ok(()) } }}
macro_rules! fn_err         { (               $error:expr ) => { Err($crate::Error(($error).into())) } }
//macro_rules! fn_error       { (               $error:expr ) => {     $crate::Error(($error).into())  } }
macro_rules! fn_param_error { ( $_param:expr, $error:expr ) => {     $crate::Error(($error).into())  } }
macro_rules! fn_error_gle   { (                           ) => {     $crate::Error::new_gle()        } }

macro_rules! mods {
    ( $( #[$attr:meta] )* inl      mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod;                       #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* inl      mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod { mods!{ $($body)* } } #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod;                                                                 mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod { mods!{ $($body)* } }                                           mods!{ $($tt)* } };
    () => {};
}

/// ### Usage
/// ```no_compile
/// convert!(RECT <=> unsafe { winapi::shared::windef::RECT, winapi::shared::windef::RECTL, winapi::shared::d3d9types::D3DRECT });
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
