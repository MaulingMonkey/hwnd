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

macro_rules! convert { ( RECT <=> unsafe { $($path:path),+ $(,)? } ) => {
    $(
        impl From<$path> for RECT { fn from(r: $path) -> Self { unsafe { std::mem::transmute(r) } } }
        impl From<RECT> for $path { fn from(r: RECT ) -> Self { unsafe { std::mem::transmute(r) } } }
        impl AsRef<$path> for RECT { fn as_ref(&self) -> &$path { unsafe { std::mem::transmute(self) } } }
        impl AsRef<RECT> for $path { fn as_ref(&self) -> &RECT  { unsafe { std::mem::transmute(self) } } }
        impl AsMut<$path> for RECT { fn as_mut(&mut self) -> &mut $path { unsafe { std::mem::transmute(self) } } }
        impl AsMut<RECT> for $path { fn as_mut(&mut self) -> &mut RECT  { unsafe { std::mem::transmute(self) } } }
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
