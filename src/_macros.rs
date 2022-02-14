macro_rules! fn_context     { ( $cpp:path => $rust:path ) => {} }
macro_rules! fn_succeeded   { ( $expr:expr ) => { if 0 == ($expr) { Err($crate::Error::new_gle()) } else { Ok(()) } }}

macro_rules! mods {
    ( $( #[$attr:meta] )* inl      mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod;                       #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* inl      mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* pub(crate) mod $mod { mods!{ $($body)* } } #[allow(unused_imports)] pub use $mod::*; mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident ;                $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod;                                                                 mods!{ $($tt)* } };
    ( $( #[$attr:meta] )* $vis:vis mod $mod:ident { $($body:tt)* } $($tt:tt)* ) => { $(#[$attr])* $vis mod $mod { mods!{ $($body)* } }                                           mods!{ $($tt)* } };
    () => {};
}
