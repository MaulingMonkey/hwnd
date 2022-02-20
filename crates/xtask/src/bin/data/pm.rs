macro_rules! flags { ( $($cpp:path),* $(,)? ) => {
    pub fn cpp_rust_values() -> impl Iterator<Item = (&'static str, &'static str, u32)> {
        use winapi::um::winuser::*;
        vec![$(
            (stringify!($cpp), stringify!($cpp).strip_prefix("PM_").unwrap(), $cpp),
        )*].into_iter()
    }
}}

flags! {
    PM_NOREMOVE,
    PM_REMOVE,
    PM_NOYIELD,
    PM_QS_INPUT,
    PM_QS_PAINT,
    PM_QS_POSTMESSAGE,
    PM_QS_SENDMESSAGE,
}
