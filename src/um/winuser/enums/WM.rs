//! WM_\* window message and notification types

#![allow(non_snake_case)]

use bytemuck::*;
use winapi::um::winuser::*;
use std::fmt::{self, Debug, Formatter};



/// WM_\* (32-bit) window message and notification types
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Pod, Zeroable)] #[repr(transparent)] pub struct WM32(u32);

impl WM32 {
    #[doc(hidden)] pub const fn from_constant(wm: u32) -> Self { Self(wm) }
    pub const fn to_u32(self) -> u32 { self.0 }
}

impl From<WM32> for u32 { fn from(cmd: WM32) -> Self { cmd.0 } }
impl From<u32> for WM32 { fn from(cmd: u32 ) -> Self { Self(cmd) } }

impl PartialEq<u32> for WM32 { fn eq(&self, other: &u32 ) -> bool { self.0 == *other } }
impl PartialEq<WM32> for u32 { fn eq(&self, other: &WM32) -> bool { *self == other.0 } }

impl Debug for WM32 {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-app#remarks
        const WM_USER_1 : u32 = WM_USER - 1;
        match self.0 {
            0       ..= WM_USER_1   => if let Some(s) = self.to_str() { write!(fmt, "{}", s) } else { write!(fmt, "WM_??? (system message {:#X})", self.0) },
            WM_USER ..= 0x7FFF      => write!(fmt, "WM_USER+{}", self.0 - WM_USER),
            WM_APP  ..= 0xBFFF      => write!(fmt, "WM_APP+{}",  self.0 - WM_APP),
            0xC000  ..= 0xFFFF      => write!(fmt, "WM_??? (string message {:#X})", self.0),
            0x10000 ..              => write!(fmt, "WM_??? (system message {:#X})", self.0),
        }
    }
}

macro_rules! messages {($(
    $(#[$($attr:meta),*$(,)?])*
    $($url:literal)?
    $ident:ident => $winapi:expr
),* $(,)? ) => {
    impl WM32 {
        fn to_str(self) -> Option<&'static str> {
            //#[allow(unreachable_patterns)]
            match self {
                $(
                    $(#[$($attr),*])*
                    $ident => Some(concat!("WM::", stringify!($ident))),
                )*
                _ => None,
            }
        }
    }

    $(
        $( #[doc = concat!("\\[[docs.microsoft.com](", $url, ")\\]")] )?
        #[doc = stringify!($winapi)]
        pub const $ident : WM32 = WM32({
            #[allow(unused_imports)] use winapi::um::winuser::*; // prioritize winuser::* over mod::*
            $winapi
        });
    )*
}}

// TODO: WM_{APP, USER} namespaces
// TODO: WM_{KEY, IME_KEY, MOUSE, TABLET, HANDHELD, AFX, PENWIN}{FIRST, LAST}

// https://social.msdn.microsoft.com/Forums/windowsapps/en-US/f677f319-9f02-4438-92fb-6e776924425d/windowproc-and-messages-0x90-0x91-0x92-0x93?forum=windowsuidevelopment
const WM_UAHDESTROYWINDOW       : u32 = 0x0090;
const WM_UAHDRAWMENU            : u32 = 0x0091;
const WM_UAHDRAWMENUITEM        : u32 = 0x0092;
const WM_UAHINITMENU            : u32 = 0x0093;
const WM_UAHMEASUREMENUITEM     : u32 = 0x0094;
const WM_UAHNCPAINTMENUPOPUP    : u32 = 0x0095;

messages! {
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-null"                 NULL                        => WM_NULL,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-create"               CREATE                      => WM_CREATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-destroy"              DESTROY                     => WM_DESTROY,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-move"                 MOVE                        => WM_MOVE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-size"                 SIZE                        => WM_SIZE,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-activate"           ACTIVATE                    => WM_ACTIVATE,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-setfocus"           SETFOCUS                    => WM_SETFOCUS,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-killfocus"          KILLFOCUS                   => WM_KILLFOCUS,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-enable"               ENABLE                      => WM_ENABLE,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-setredraw"               SETREDRAW                   => WM_SETREDRAW,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-settext"              SETTEXT                     => WM_SETTEXT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-gettext"              GETTEXT                     => WM_GETTEXT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-gettextlength"        GETTEXTLENGTH               => WM_GETTEXTLENGTH,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paint"                   PAINT                       => WM_PAINT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-close"                CLOSE                       => WM_CLOSE,
    "https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-queryendsession"    QUERYENDSESSION             => WM_QUERYENDSESSION,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-queryopen"            QUERYOPEN                   => WM_QUERYOPEN,
    "https://docs.microsoft.com/en-us/windows/win32/shutdown/wm-endsession"         ENDSESSION                  => WM_ENDSESSION,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-quit"                 QUIT                        => WM_QUIT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-erasebkgnd"           ERASEBKGND                  => WM_ERASEBKGND,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-syscolorchange"          SYSCOLORCHANGE              => WM_SYSCOLORCHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-showwindow"           SHOWWINDOW                  => WM_SHOWWINDOW,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-wininichange"         WININICHANGE                => WM_WININICHANGE,
    #[allow(unreachable_patterns)]
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-settingchange"        SETTINGCHANGE               => WM_SETTINGCHANGE,                            // redundant

    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-devmodechange"           DEVMODECHANGE               => WM_DEVMODECHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-activateapp"          ACTIVATEAPP                 => WM_ACTIVATEAPP,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-fontchange"              FONTCHANGE                  => WM_FONTCHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/sysinfo/wm-timechange"          TIMECHANGE                  => WM_TIMECHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-cancelmode"           CANCELMODE                  => WM_CANCELMODE,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-setcursor"            SETCURSOR                   => WM_SETCURSOR,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mouseactivate"      MOUSEACTIVATE               => WM_MOUSEACTIVATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-childactivate"        CHILDACTIVATE               => WM_CHILDACTIVATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-queuesync"            QUEUESYNC                   => WM_QUEUESYNC,

    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getminmaxinfo"        GETMINMAXINFO               => WM_GETMINMAXINFO,

    /* undocumented by microsoft? */                                                PAINTICON                   => WM_PAINTICON,
    /* undocumented by microsoft? */                                                ICONERASEBKGND              => WM_ICONERASEBKGND,
    "https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-nextdlgctl"           NEXTDLGCTL                  => WM_NEXTDLGCTL,
    "https://docs.microsoft.com/en-us/windows/win32/printdocs/wm-spoolerstatus"     SPOOLERSTATUS               => WM_SPOOLERSTATUS,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-drawitem"           DRAWITEM                    => WM_DRAWITEM,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-measureitem"        MEASUREITEM                 => WM_MEASUREITEM,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-deleteitem"         DELETEITEM                  => WM_DELETEITEM,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-vkeytoitem"         VKEYTOITEM                  => WM_VKEYTOITEM,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-chartoitem"         CHARTOITEM                  => WM_CHARTOITEM,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-setfont"              SETFONT                     => WM_SETFONT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-getfont"              GETFONT                     => WM_GETFONT,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-sethotkey"          SETHOTKEY                   => WM_SETHOTKEY,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-gethotkey"          GETHOTKEY                   => WM_GETHOTKEY,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-querydragicon"        QUERYDRAGICON               => WM_QUERYDRAGICON,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-compareitem"        COMPAREITEM                 => WM_COMPAREITEM,
    "https://docs.microsoft.com/en-us/windows/win32/winauto/wm-getobject"           GETOBJECT                   => WM_GETOBJECT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-compacting"           COMPACTING                  => WM_COMPACTING,
    /* undocumented by microsoft? */                                                COMMNOTIFY                  => WM_COMMNOTIFY,                               // no longer suported
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanging"    WINDOWPOSCHANGING           => WM_WINDOWPOSCHANGING,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-windowposchanged"     WINDOWPOSCHANGED            => WM_WINDOWPOSCHANGED,

    "https://docs.microsoft.com/en-us/windows/win32/power/wm-power"                 POWER                       => WM_POWER,

    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-copydata"           COPYDATA                    => WM_COPYDATA,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-canceljournal"        CANCELJOURNAL               => WM_CANCELJOURNAL,

    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-notify"             NOTIFY                      => WM_NOTIFY,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-inputlangchangerequest" INPUTLANGCHANGEREQUEST      => WM_INPUTLANGCHANGEREQUEST,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-inputlangchange"      INPUTLANGCHANGE             => WM_INPUTLANGCHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/shell/wm-tcard"                 TCARD                       => WM_TCARD,
    "https://docs.microsoft.com/en-us/windows/win32/shell/wm-help"                  HELP                        => WM_HELP,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-userchanged"          USERCHANGED                 => WM_USERCHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-notifyformat"       NOTIFYFORMAT                => WM_NOTIFYFORMAT,

    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-contextmenu"          CONTEXTMENU                 => WM_CONTEXTMENU,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanging"        STYLECHANGING               => WM_STYLECHANGING,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-stylechanged"         STYLECHANGED                => WM_STYLECHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-displaychange"           DISPLAYCHANGE               => WM_DISPLAYCHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-geticon"              GETICON                     => WM_GETICON,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-seticon"              SETICON                     => WM_SETICON,

    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccreate"             NCCREATE                    => WM_NCCREATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncdestroy"            NCDESTROY                   => WM_NCDESTROY,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-nccalcsize"           NCCALCSIZE                  => WM_NCCALCSIZE,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nchittest"          NCHITTEST                   => WM_NCHITTEST,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-ncpaint"                 NCPAINT                     => WM_NCPAINT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-ncactivate"           NCACTIVATE                  => WM_NCACTIVATE,
    "https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-getdlgcode"           GETDLGCODE                  => WM_GETDLGCODE,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-syncpaint"               SYNCPAINT                   => WM_SYNCPAINT,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmousemove"        NCMOUSEMOVE                 => WM_NCMOUSEMOVE,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttondown"      NCLBUTTONDOWN               => WM_NCLBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttonup"        NCLBUTTONUP                 => WM_NCLBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-nclbuttondblclk"    NCLBUTTONDBLCLK             => WM_NCLBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttondown"      NCRBUTTONDOWN               => WM_NCRBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttonup"        NCRBUTTONUP                 => WM_NCRBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncrbuttondblclk"    NCRBUTTONDBLCLK             => WM_NCRBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttondown"      NCMBUTTONDOWN               => WM_NCMBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttonup"        NCMBUTTONUP                 => WM_NCMBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmbuttondblclk"    NCMBUTTONDBLCLK             => WM_NCMBUTTONDBLCLK,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncXbuttondown"      NCXBUTTONDOWN               => WM_NCXBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncXbuttonup"        NCXBUTTONUP                 => WM_NCXBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncXbuttondblclk"    NCXBUTTONDBLCLK             => WM_NCXBUTTONDBLCLK,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-input-device-change" INPUT_DEVICE_CHANGE        => WM_INPUT_DEVICE_CHANGE,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-input"              INPUT                       => WM_INPUT,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keydown"            KEYDOWN                     => WM_KEYDOWN,                                  // WM_KEYFIRST
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-keyup"              KEYUP                       => WM_KEYUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-char"               CHAR                        => WM_CHAR,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-deadchar"           DEADCHAR                    => WM_DEADCHAR,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeydown"         SYSKEYDOWN                  => WM_SYSKEYDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-syskeyup"           SYSKEYUP                    => WM_SYSKEYUP,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syschar"              SYSCHAR                     => WM_SYSCHAR,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-sysdeadchar"        SYSDEADCHAR                 => WM_SYSDEADCHAR,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-unichar"            UNICHAR                     => WM_UNICHAR,                                  // WM_KEYLAST

    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-startcomposition"   IME_STARTCOMPOSITION        => WM_IME_STARTCOMPOSITION,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-endcomposition"     IME_ENDCOMPOSITION          => WM_IME_ENDCOMPOSITION,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-composition"        IME_COMPOSITION             => WM_IME_COMPOSITION,                          // WM_IME_KEYLAST

    "https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-initdialog"           INITDIALOG                  => WM_INITDIALOG,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-command"              COMMAND                     => WM_COMMAND,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-syscommand"           SYSCOMMAND                  => WM_SYSCOMMAND,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-timer"                TIMER                       => WM_TIMER,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-hscroll"            HSCROLL                     => WM_HSCROLL,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-vscroll"            VSCROLL                     => WM_VSCROLL,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenu"             INITMENU                    => WM_INITMENU,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-initmenupopup"        INITMENUPOPUP               => WM_INITMENUPOPUP,
    "https://docs.microsoft.com/en-us/windows/win32/wintouch/wm-gesture"            GESTURE                     => WM_GESTURE,
    "https://docs.microsoft.com/en-us/windows/win32/wintouch/wm-gesturenotify"      GESTURENOTIFY               => WM_GESTURENOTIFY,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menuselect"           MENUSELECT                  => WM_MENUSELECT,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menuchar"             MENUCHAR                    => WM_MENUCHAR,
    "https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-enteridle"            ENTERIDLE                   => WM_ENTERIDLE,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menurbuttonup"        MENURBUTTONUP               => WM_MENURBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menudrag"             MENUDRAG                    => WM_MENUDRAG,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menugetobject"        MENUGETOBJECT               => WM_MENUGETOBJECT,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-uninitmenupopup"      UNINITMENUPOPUP             => WM_UNINITMENUPOPUP,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-menucommand"          MENUCOMMAND                 => WM_MENUCOMMAND,

    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-changeuistate"        CHANGEUISTATE               => WM_CHANGEUISTATE,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-updateuistate"        UPDATEUISTATE               => WM_UPDATEUISTATE,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-queryuistate"         QUERYUISTATE                => WM_QUERYUISTATE,

    /* undocumented by microsoft? */                                                CTLCOLORMSGBOX              => WM_CTLCOLORMSGBOX,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcoloredit"       CTLCOLOREDIT                => WM_CTLCOLOREDIT,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorlistbox"    CTLCOLORLISTBOX             => WM_CTLCOLORLISTBOX,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorbtn"        CTLCOLORBTN                 => WM_CTLCOLORBTN,
    "https://docs.microsoft.com/en-us/windows/win32/dlgbox/wm-ctlcolordlg"          CTLCOLORDLG                 => WM_CTLCOLORDLG,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorscrollbar"  CTLCOLORSCROLLBAR           => WM_CTLCOLORSCROLLBAR,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-ctlcolorstatic"     CTLCOLORSTATIC              => WM_CTLCOLORSTATIC,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/mn-gethmenu"             MN_GETHMENU                 => MN_GETHMENU,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousemove"          MOUSEMOVE                   => WM_MOUSEMOVE,                                // WM_MOUSEFIRST
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondown"        LBUTTONDOWN                 => WM_LBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttonup"          LBUTTONUP                   => WM_LBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk"      LBUTTONDBLCLK               => WM_LBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondown"        RBUTTONDOWN                 => WM_RBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttonup"          RBUTTONUP                   => WM_RBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-rbuttondblclk"      RBUTTONDBLCLK               => WM_RBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondown"        MBUTTONDOWN                 => WM_MBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttonup"          MBUTTONUP                   => WM_MBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mbuttondblclk"      MBUTTONDBLCLK               => WM_MBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousewheel"         MOUSEWHEEL                  => WM_MOUSEWHEEL,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondown"        XBUTTONDOWN                 => WM_XBUTTONDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttonup"          XBUTTONUP                   => WM_XBUTTONUP,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-xbuttondblclk"      XBUTTONDBLCLK               => WM_XBUTTONDBLCLK,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehwheel"        MOUSEHWHEEL                 => WM_MOUSEHWHEEL,                              // ~ WM_MOUSELAST?

    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-parentnotify"               PARENTNOTIFY  => WM_PARENTNOTIFY,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-entermenuloop"                                    ENTERMENULOOP               => WM_ENTERMENULOOP,
    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-exitmenuloop"                                     EXITMENULOOP                => WM_EXITMENULOOP,

    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-nextmenu"                                         NEXTMENU                    => WM_NEXTMENU,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-sizing"                                           SIZING                      => WM_SIZING,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-capturechanged"                                 CAPTURECHANGED              => WM_CAPTURECHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-moving"                                           MOVING                      => WM_MOVING,

    "https://docs.microsoft.com/en-us/windows/win32/power/wm-powerbroadcast"                                    POWERBROADCAST              => WM_POWERBROADCAST,

    "https://docs.microsoft.com/en-us/windows/win32/devio/wm-devicechange"                                      DEVICECHANGE                => WM_DEVICECHANGE,

    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdicreate"                                        MDICREATE                   => WM_MDICREATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdidestroy"                                       MDIDESTROY                  => WM_MDIDESTROY,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdiactivate"                                      MDIACTIVATE                 => WM_MDIACTIVATE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdirestore"                                       MDIRESTORE                  => WM_MDIRESTORE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdinext"                                          MDINEXT                     => WM_MDINEXT,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdimaximize"                                      MDIMAXIMIZE                 => WM_MDIMAXIMIZE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mditile"                                          MDITILE                     => WM_MDITILE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdicascade"                                       MDICASCADE                  => WM_MDICASCADE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdiiconarrange"                                   MDIICONARRANGE              => WM_MDIICONARRANGE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdigetactive"                                     MDIGETACTIVE                => WM_MDIGETACTIVE,

    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdisetmenu"                                       MDISETMENU                  => WM_MDISETMENU,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-entersizemove"                                    ENTERSIZEMOVE               => WM_ENTERSIZEMOVE,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-exitsizemove"                                     EXITSIZEMOVE                => WM_EXITSIZEMOVE,
    "https://docs.microsoft.com/en-us/windows/win32/shell/wm-dropfiles"                                         DROPFILES                   => WM_DROPFILES,
    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-mdirefreshmenu"                                   MDIREFRESHMENU              => WM_MDIREFRESHMENU,

    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerdevicechange"        POINTERDEVICECHANGE         => WM_POINTERDEVICECHANGE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerdeviceinrange"       POINTERDEVICEINRANGE        => WM_POINTERDEVICEINRANGE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerdeviceoutofrange"    POINTERDEVICEOUTOFRANGE     => WM_POINTERDEVICEOUTOFRANGE,

    "https://docs.microsoft.com/en-us/windows/win32/wintouch/wm-touchdown"                                      TOUCH                       => WM_TOUCH,

    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-ncpointerupdate"            NCPOINTERUPDATE             => WM_NCPOINTERUPDATE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-ncpointerdown"              NCPOINTERDOWN               => WM_NCPOINTERDOWN,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-ncpointerup"                NCPOINTERUP                 => WM_NCPOINTERUP,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerupdate"              POINTERUPDATE               => WM_POINTERUPDATE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerdown"                POINTERDOWN                 => WM_POINTERDOWN,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerup"                  POINTERUP                   => WM_POINTERUP,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerenter"               POINTERENTER                => WM_POINTERENTER,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerleave"               POINTERLEAVE                => WM_POINTERLEAVE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointeractivate"            POINTERACTIVATE             => WM_POINTERACTIVATE,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointercapturechanged"      POINTERCAPTURECHANGED       => WM_POINTERCAPTURECHANGED,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-touchhittesting"            TOUCHHITTESTING             => WM_TOUCHHITTESTING,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerwheel"               POINTERWHEEL                => WM_POINTERWHEEL,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerhwheel"              POINTERHWHEEL               => WM_POINTERHWHEEL,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/dm-pointerhittest"             DM_POINTERHITTEST           => DM_POINTERHITTEST, // XXX: funky!
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerroutedto"            POINTERROUTEDTO             => WM_POINTERROUTEDTO,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerroutedaway"          POINTERROUTEDAWAY           => WM_POINTERROUTEDAWAY,
    "https://docs.microsoft.com/en-us/previous-versions/windows/desktop/inputmsg/wm-pointerroutedreleased"      POINTERROUTEDRELEASED       => WM_POINTERROUTEDRELEASED,

    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-setcontext"                 IME_SETCONTEXT              => WM_IME_SETCONTEXT,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-notify"                     IME_NOTIFY                  => WM_IME_NOTIFY,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-control"                    IME_CONTROL                 => WM_IME_CONTROL,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-compositionfull"            IME_COMPOSITIONFULL         => WM_IME_COMPOSITIONFULL,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-select"                     IME_SELECT                  => WM_IME_SELECT,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-char"                       IME_CHAR                    => WM_IME_CHAR,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-request"                    IME_REQUEST                 => WM_IME_REQUEST,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-keydown"                    IME_KEYDOWN                 => WM_IME_KEYDOWN,
    "https://docs.microsoft.com/en-us/windows/win32/intl/wm-ime-keyup"                      IME_KEYUP                   => WM_IME_KEYUP,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mousehover"                 MOUSEHOVER                  => WM_MOUSEHOVER,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-mouseleave"                 MOUSELEAVE                  => WM_MOUSELEAVE,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmousehover"               NCMOUSEHOVER                => WM_NCMOUSEHOVER,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-ncmouseleave"               NCMOUSELEAVE                => WM_NCMOUSELEAVE,

    "https://docs.microsoft.com/en-us/windows/win32/termserv/wm-wtssession-change"          WTSSESSION_CHANGE           => WM_WTSSESSION_CHANGE,

    "https://docs.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged"                    DPICHANGED                  => WM_DPICHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged-beforeparent"       DPICHANGED_BEFOREPARENT     => WM_DPICHANGED_BEFOREPARENT,
    "https://docs.microsoft.com/en-us/windows/win32/hidpi/wm-dpichanged-afterparent"        DPICHANGED_AFTERPARENT      => WM_DPICHANGED_AFTERPARENT,
    "https://docs.microsoft.com/en-us/windows/win32/hidpi/wm-getdpiscaledsize"              GETDPISCALEDSIZE            => WM_GETDPISCALEDSIZE,

    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-cut"                        CUT                         => WM_CUT,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-copy"                       COPY                        => WM_COPY,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-paste"                      PASTE                       => WM_PASTE,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-clear"                      CLEAR                       => WM_CLEAR,
    "https://docs.microsoft.com/en-us/windows/win32/controls/wm-undo"                       UNDO                        => WM_UNDO,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-renderformat"               RENDERFORMAT                => WM_RENDERFORMAT,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-renderallformats"           RENDERALLFORMATS            => WM_RENDERALLFORMATS,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-destroyclipboard"           DESTROYCLIPBOARD            => WM_DESTROYCLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-drawclipboard"              DRAWCLIPBOARD               => WM_DRAWCLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-paintclipboard"             PAINTCLIPBOARD              => WM_PAINTCLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-vscrollclipboard"           VSCROLLCLIPBOARD            => WM_VSCROLLCLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-sizeclipboard"              SIZECLIPBOARD               => WM_SIZECLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-askcbformatname"            ASKCBFORMATNAME             => WM_ASKCBFORMATNAME,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-changecbchain"              CHANGECBCHAIN               => WM_CHANGECBCHAIN,
    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-hscrollclipboard"           HSCROLLCLIPBOARD            => WM_HSCROLLCLIPBOARD,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-querynewpalette"                 QUERYNEWPALETTE             => WM_QUERYNEWPALETTE,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-paletteischanging"               PALETTEISCHANGING           => WM_PALETTEISCHANGING,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-palettechanged"                  PALETTECHANGED              => WM_PALETTECHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-hotkey"                     HOTKEY                      => WM_HOTKEY,

    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-print"                           PRINT                       => WM_PRINT,
    "https://docs.microsoft.com/en-us/windows/win32/gdi/wm-printclient"                     PRINTCLIENT                 => WM_PRINTCLIENT,

    "https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-appcommand"                 APPCOMMAND                  => WM_APPCOMMAND,

    "https://docs.microsoft.com/en-us/windows/win32/winmsg/wm-themechanged"                 THEMECHANGED                => WM_THEMECHANGED,

    "https://docs.microsoft.com/en-us/windows/win32/dataxchg/wm-clipboardupdate"            CLIPBOARDUPDATE             => WM_CLIPBOARDUPDATE,

    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmcompositionchanged"           DWMCOMPOSITIONCHANGED       => WM_DWMCOMPOSITIONCHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmncrenderingchanged"           DWMNCRENDERINGCHANGED       => WM_DWMNCRENDERINGCHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmcolorizationcolorchanged"     DWMCOLORIZATIONCOLORCHANGED => WM_DWMCOLORIZATIONCOLORCHANGED,
    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmwindowmaximizedchange"        DWMWINDOWMAXIMIZEDCHANGE    => WM_DWMWINDOWMAXIMIZEDCHANGE,

    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmsendiconicthumbnail"          DWMSENDICONICTHUMBNAIL      => WM_DWMSENDICONICTHUMBNAIL,
    "https://docs.microsoft.com/en-us/windows/win32/dwm/wm-dwmsendiconiclivepreviewbitmap"  DWMSENDICONICLIVEPREVIEWBITMAP => WM_DWMSENDICONICLIVEPREVIEWBITMAP,

    "https://docs.microsoft.com/en-us/windows/win32/menurc/wm-gettitlebarinfoex"            GETTITLEBARINFOEX           => WM_GETTITLEBARINFOEX,

    /* undocumented by microsoft */                                                         UAHDESTROYWINDOW            => WM_UAHDESTROYWINDOW,
    /* undocumented by microsoft */                                                         UAHDRAWMENU                 => WM_UAHDRAWMENU,
    /* undocumented by microsoft */                                                         UAHDRAWMENUITEM             => WM_UAHDRAWMENUITEM,
    /* undocumented by microsoft */                                                         UAHINITMENU                 => WM_UAHINITMENU,
    /* undocumented by microsoft */                                                         UAHMEASUREMENUITEM          => WM_UAHMEASUREMENUITEM,
    /* undocumented by microsoft */                                                         UAHNCPAINTMENUPOPUP         => WM_UAHNCPAINTMENUPOPUP,
}
