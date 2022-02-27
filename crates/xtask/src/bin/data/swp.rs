macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("SWP_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    SWP_NOSIZE,
    SWP_NOMOVE,
    SWP_NOZORDER,
    SWP_NOREDRAW,
    SWP_NOACTIVATE,
    SWP_FRAMECHANGED,
    SWP_SHOWWINDOW,
    SWP_HIDEWINDOW,
    SWP_NOCOPYBITS,
    SWP_NOOWNERZORDER,
    SWP_NOSENDCHANGING,
    SWP_DRAWFRAME,
    SWP_NOREPOSITION,
    SWP_DEFERERASE,
    SWP_ASYNCWINDOWPOS,
}
