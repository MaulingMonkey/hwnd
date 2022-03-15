use bytemuck::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/windef/ns-windef-point)\]
/// POINT
///
/// Defines the x- and y-coordinates of a point.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Pod, Zeroable)]
#[repr(C)] pub struct Point {
    pub x: i32,
    pub y: i32,
}

convert!(Point <=> unsafe { winapi::shared::windef::POINT });
