#![doc = include_str!("../Readme.md")]
#![forbid(unsafe_op_in_unsafe_fn)]
#![deny(unreachable_patterns)] // probably improperly `match { ... }`ed constants
#![debugger_visualizer(natvis_file = "../hwnd.natvis")]

use winresult::ERROR;
#[doc(hidden)] pub use WM::WM32;
use WS::WindowStyle;
use WS_EX::WindowStyleExtended;

#[macro_use] mod _macros;

pub use winapi::shared::minwindef::LPARAM;          // OK?
pub use winapi::shared::minwindef::LRESULT;         // OK?
pub use winapi::shared::minwindef::WPARAM;          // OK?

pub use winapi::shared::windef::HWND;               // TODO: wrap / typeify
pub use winapi::shared::windef::HCURSOR;            // TODO: wrap / typeify
pub use winapi::shared::windef::HMENU;              // TODO: wrap / typeify
pub use winapi::shared::windef::HICON;              // TODO: wrap / typeify
pub use winapi::shared::windef::HBRUSH;             // TODO: wrap / typeify

#[cfg(doc)] pub mod doc {
    //! `doc/*.md` markdown documentation
    #[doc = include_str!("../doc/alternatives.md"               )] pub const Alternatives : () = ();
    #[doc = include_str!("../doc/errors.md"                     )] pub const Errors : () = ();
    #[doc = include_str!("../doc/features.md"                   )] pub const Features : () = ();
    #[doc = include_str!("../doc/unsound-assumptions.md"        )] pub const Unsound_Assumptions : () = ();
    #[doc = include_str!("../doc/window-lifecycle-events.md"    )] pub const Window_Lifecycle_Events : () = ();
}

#[path = "assoc/_assoc.rs"] pub mod assoc;

mods! {
    inl mod structures {
        inl mod error;
    }

    inl mod utils {
        inl mod _32;
        inl mod gle;
    }

    /// shared/*.h
    pub mod shared {
        /// shared/minwindef.h
        pub mod minwindef {
            inl mod extras {
                inl mod name_or_atom;
            }

            inl mod handles {
                inl mod hmodule;
            }

            inl mod values {
                inl mod atom;
            }
        }

        /// shared/windef.h
        pub mod windef {
            inl mod handles {
                inl mod hcursor;
                inl mod hicon;
                inl mod hwnd_;
            }

            inl mod structures {
                inl mod point;
                inl mod rect;
            }
        }
    }

    /// um/*.h
    pub mod um {
        /// um/libloaderapi.h
        pub mod libloaderapi {
            inl mod functions {
                inl mod get_module_handle_;
            }
        }

        /// um/processthreadsapi.h
        pub mod processthreadsapi {
            inl mod functions {
                inl mod get_current_x_id;
            }
        }

        /// um/winuser.h
        pub mod winuser {
            inl mod enums {
                pub mod GWL;
                pub mod GWLP;
                pub mod IDC;
                pub mod IDI;
                pub mod SW;
                pub mod WM;
            }

            inl mod flags {
                pub mod ISMEX;
                pub mod PM;
                pub mod SMTO;
                pub mod SWP;
                pub mod WPF;
                pub mod WS;
                pub mod WS_EX;
            }

            inl mod functions {
                inl mod adjust_window_rect_;
                inl mod close_window_;
                inl mod create_window_;
                inl mod def_window_proc;
                inl mod destroy_window_;
                inl mod dispatch_message;
                inl mod get_client_rect_;
                inl mod get_message;
                inl mod get_window_long_ptr;
                inl mod get_window_long;
                inl mod get_window_placement_;
                inl mod get_window_rect_;
                inl mod get_window_text;
                inl mod get_window_thread_process_id_;
                inl mod get_x_window;
                inl mod in_send_message_;
                inl mod is;
                inl mod load_cursor;
                inl mod load_icon;
                inl mod peek_message;
                inl mod post_message;
                inl mod register_class_;
                inl mod register_window_message;
                inl mod reply_message_;
                inl mod send_message;
                inl mod set_foreground_window_;
                inl mod set_window_placement_;
                inl mod set_window_pos_;
                inl mod set_window_text;
                inl mod show_window_;
                inl mod translate_message_;
            }

            inl mod structures {
                inl mod msg;
                inl mod window_placement;
                inl mod wndclass;
            }
        }
    }
}

#[doc(no_inline)] pub use shared::minwindef::*;
#[doc(no_inline)] pub use shared::windef::*;
#[doc(no_inline)] pub use um::libloaderapi::*;
#[doc(no_inline)] pub use um::processthreadsapi::*;
#[doc(no_inline)] pub use um::winuser::*;
