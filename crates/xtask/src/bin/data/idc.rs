macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, usize)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("IDC_").unwrap(), $cpp as usize),
        )*].into_iter()
    }
}}

// missing from winapi
pub const IDC_PIN       : *const u16 = 32671 as _; // WINVER >= 0x0606
pub const IDC_PERSON    : *const u16 = 32672 as _; // WINVER >= 0x0606

flags! {
    IDC_ARROW,
    IDC_IBEAM,
    IDC_WAIT,
    IDC_CROSS,
    IDC_UPARROW,
    IDC_SIZE,
    IDC_ICON,
    IDC_SIZENWSE,
    IDC_SIZENESW,
    IDC_SIZEWE,
    IDC_SIZENS,
    IDC_SIZEALL,
    IDC_NO,
    IDC_HAND,
    IDC_APPSTARTING,
    IDC_HELP,
    IDC_PIN,
    IDC_PERSON,
}
