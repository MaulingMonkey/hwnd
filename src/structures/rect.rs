use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect)\]
/// RECT / RECTL / D3DRECT
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Pod, Zeroable)]
#[repr(C)] pub struct RECT {
    pub left:   i32,
    pub top:    i32,
    pub right:  i32,
    pub bottom: i32,
}

macro_rules! convert { ( $self:ident <=> unsafe { $($path:path),+ $(,)? } ) => {
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

    #[test] fn layout() {
        use std::mem::*;
        $(
        assert_eq!(size_of ::<RECT>(), size_of ::<$path>());
        assert_eq!(align_of::<RECT>(), align_of::<$path>());
        )*
    }
}}

convert!(RECT <=> unsafe { winapi::shared::windef::RECT, winapi::shared::windef::RECTL, winapi::shared::d3d9types::D3DRECT });
