fn main() {
    natvis::gen();
}

mod data {
    pub mod gwl;
    pub mod gwlp;
    pub mod ismex;
    pub mod pm;
    pub mod smto;
    pub mod swp;
    pub mod wm;
    pub mod wpf;
    pub mod ws_ex;
    pub mod ws;
}

mod natvis {
    pub fn gen() {
        mmrbi::fs::write_if_modified_with("hwnd.natvis", |nv| {
            use std::io::{Write as _};
            let mut nv = mmrbi::io::EolRewriter(nv);

            writeln!(nv, r#"<?xml version="1.0" encoding="utf-8"?>"#)?;
            writeln!(nv, r#"<!-- WARNING: this file is auto-generated by xtask gen and may be overwritten -->"#)?;
            writeln!(nv)?;
            writeln!(nv, r#"<AutoVisualizer xmlns="http://schemas.microsoft.com/vstudio/debugger/natvis/2010">"#)?;

            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::shared::minwindef::handles::hmodule::HModule">"#)?;
            writeln!(nv, r#"        <DisplayString>{{(HMODULE)__0}}</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;
            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::shared::windef::handles::hwnd_::HWnd">"#)?;
            writeln!(nv, r#"        <DisplayString>HWnd({{__0,X}})</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;
            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::shared::windef::handles::hcursor::HCursor">"#)?;
            writeln!(nv, r#"        <DisplayString>HCursor({{__0,X}})</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;
            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::shared::windef::handles::hicon::HIcon">"#)?;
            writeln!(nv, r#"        <DisplayString>HIcon({{__0,X}})</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;

            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::um::winuser::enums::WM::WM16">"#)?; // doesn't exist yet
            writeln!(nv, r#"        <DisplayString>{{__0,wm}}</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;

            writeln!(nv)?;
            writeln!(nv, r#"    <Type Name="hwnd::um::winuser::enums::WM::WM32">"#)?;
            writeln!(nv, r#"        <DisplayString>{{__0,wm}}</DisplayString>"#)?;
            writeln!(nv, r#"    </Type>"#)?;



            // enum-style enums
            for (ty, pre, values) in vec![
                ("hwnd::um::winuser::enums::GWL::GetWindowLongIndex",        "GWL",      crate::data::gwl    ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::enums::GWL::GetWindowLongPtrIndex",     "GWLP",     crate::data::gwlp   ::cpp_rust_values().collect::<Vec<_>>()),
                // IDC
                // SW
                ("hwnd::um::winuser::enums::WM::WM32",                       "WM",       crate::data::wm     ::cpp_rust_values().collect::<Vec<_>>()),
            ].into_iter() {
                writeln!(nv)?;
                writeln!(nv, r#"    <Type Name="{ty}">"#)?;
                for (_cpp, rust, value) in values.iter() {
                    writeln!(nv, r#"        <DisplayString Condition="__0 == {value}">{rust}</DisplayString>"#)?;
                }
                writeln!(nv, r#"        <DisplayString>{{__0}} ({pre}::???)</DisplayString>"#)?;
                writeln!(nv, r#"    </Type>"#)?;
            }



            // flag-style enums
            for (ty, pre, values) in vec![
                ("hwnd::um::winuser::flags::ISMEX::Flags",                   "ISMEX",    crate::data::ismex  ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::PM::PeekMessageFlags",           "PM",       crate::data::pm     ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::SMTO::SendMessageTimeOutFlags",  "SMTO",     crate::data::smto   ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::SWP::SetWindowPosFlags",         "SWP",      crate::data::swp    ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::WPF::WindowPlacementFlags",      "WPF",      crate::data::wpf    ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::WS::WindowStyle",                "WS",       crate::data::ws     ::cpp_rust_values().collect::<Vec<_>>()),
                ("hwnd::um::winuser::flags::WS_EX::WindowStyleExtended",     "WS_EX",    crate::data::ws_ex  ::cpp_rust_values().collect::<Vec<_>>()),
            ].into_iter() {
                writeln!(nv)?;
                writeln!(nv, r#"    <Type Name="{ty}">"#)?;
                for (_cpp, rust, value) in values.iter() {
                    writeln!(nv, r#"        <DisplayString Condition="0x{value:08X} == __0">{pre}::{rust}</DisplayString>"#)?;
                }
                writeln!(nv, r#"        <DisplayString Condition="__0 == 0">{pre}::{{{{0}}}}</DisplayString>"#)?;
                writeln!(nv, r#"        <DisplayString ExcludeView="truelist">{pre}::{{*this,view(truelist)}}</DisplayString>"#)?;
                writeln!(nv, r#"        <Expand>"#)?;
                for (_cpp, rust, value) in values.iter() {
                    if *value == 0 { continue }
                    let qrust = format!("\"{pre}::{rust}\"");
                    writeln!(nv, r#"            <Item Name={qrust: <32} ExcludeView="truelist" Condition="0x{value:08X} == (__0 &amp; 0x{value:08X})">true</Item>"#)?;
                    writeln!(nv, r#"            <Item Name={qrust: <32} ExcludeView="truelist" Condition="0x{value:08X} != (__0 &amp; 0x{value:08X})">0</Item>"#)?;
                }
                writeln!(nv, r#"            <CustomListItems MaxItemsPerView="64" IncludeView="truelist">"#)?;
                for (_cpp, rust, value) in values.iter() {
                    if *value == 0 { continue }
                    writeln!(nv, r#"                <Item Condition="0x{value:08X} == (__0 &amp; 0x{value:08X})">"{rust}",sb</Item>"#)?;
                }
                writeln!(nv, r#"            </CustomListItems>"#)?;
                writeln!(nv, r#"        </Expand>"#)?;
                writeln!(nv, r#"    </Type>"#)?;
            }



            writeln!(nv)?;
            writeln!(nv, r#"</AutoVisualizer>"#)
        }).unwrap();
    }
}
