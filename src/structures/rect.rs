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

convert!(RECT <=> unsafe { winapi::shared::windef::RECT, winapi::shared::windef::RECTL, winapi::shared::d3d9types::D3DRECT });
