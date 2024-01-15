//! Associate data with an [HWnd] in a thread-local manner.
//!
//! ### Common Errors
//! *   [ERROR::INVALID_WINDOW_HANDLE]      if an [HWnd] is invalid (or, for "early" slots, is being destroyed)
//! *   [ERROR::WINDOW_OF_OTHER_THREAD]     if an [HWnd] belongs to another thread or process
//! *   [ERROR::DATATYPE_MISMATCH]          if the existing/previous value associated with a [HWnd] didn't match the expected type (a bug?)
//!
//! ### Example
//! ```
//! # use hwnd::*;
//! # use winresult::*;
//! # use std::ptr::*;
//! # let local_process_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
//! # std::thread::spawn(move ||{
//! # let local_thread_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
//! #
//! use hwnd::assoc::local::*;
//!
//! static SLOT : Slot<&'static str> = Slot::new_drop_late();
//!
//! # #[cfg(xxx)] {
//! let local_thread_hwnd   : HWnd = ..; // hwnd belonging to local thread
//! let local_process_hwnd  : HWnd = ..; // hwnd belonging to local process, another thread
//! # }
//! let remote_process_hwnd : HWnd = get_desktop_window();
//!
//! assert_eq!(None,                          SLOT.set(local_thread_hwnd, "init").unwrap());
//! assert_eq!(Some("init"),                  SLOT.set(local_thread_hwnd, "slot").unwrap());
//! assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.set(local_process_hwnd,   "X").unwrap_err());
//! assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.set(remote_process_hwnd,  "X").unwrap_err());
//! assert_eq!(ERROR::INVALID_WINDOW_HANDLE,  SLOT.set(HWnd::NULL,           "X").unwrap_err());
//!
//! assert_eq!(Some("slot"),                  SLOT.get_copy(local_thread_hwnd    ).unwrap());
//! assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_copy(local_process_hwnd   ).unwrap_err());
//! assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_copy(remote_process_hwnd  ).unwrap_err());
//! assert_eq!(ERROR::INVALID_WINDOW_HANDLE,  SLOT.get_copy(HWnd::NULL           ).unwrap_err());
//! #
//! # }).join().unwrap();
//! ```

use crate::*;

use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::any::*;
use std::cell::*;
use std::collections::*;
use std::marker::*;
use std::ptr::*;
use std::sync::{*, atomic::{*, Ordering::*}};



/// A typed slot that can be used to set or retrieve data associated with an [HWnd].
pub struct Slot<T: 'static> {
    ty:             SlotType,
    dense_slot_no:  AtomicUsize, // 0 => not set, N => dense_slot_idx = N-1
    pd:             PhantomData<std::thread::LocalKey<&'static T>>,
}

impl<T: 'static> Slot<T> {
    /// Associated data will be dropped early (before [WM::DESTROY] or [WM::NCDESTROY]) when a window is [destroyed](destroy_window).
    ///
    /// Said data will, obviously, not be accessible from within [WM::DESTROY], [WM::NCDESTROY], or similar events as a result.
    /// This can sanely be used for e.g. references to Direct3D devices and resources that should simply be dropped before a window is destroyed.
    ///
    /// ### Example
    /// ```
    /// # use hwnd::*;
    /// use hwnd::assoc::local::*;
    ///
    /// static SLOT : Slot<&'static str> = Slot::new_drop_early();
    /// ```
    pub const fn new_drop_early() -> Self { Self::new_impl(SlotType::DenseDropEarly) }

    /// Associated data will be dropped late (after [WM::DESTROY] and [WM::NCDESTROY]) when a window is [destroyed](destroy_window).
    ///
    /// Said data will generally be accessible from within [WM::DESTROY] or [WM::NCDESTROY].
    /// You might use this for data you need to manually handle the destruction of within [WM::DESTROY].
    /// It's worth noting that in some edge cases (e.g. when calling [destroy_window] from within a [destroy_window] of the same hwnd) the data will have been removed anyways.
    ///
    /// ### Example
    /// ```
    /// # use hwnd::*;
    /// use hwnd::assoc::local::*;
    ///
    /// static SLOT : Slot<&'static str> = Slot::new_drop_late();
    /// ```
    pub const fn new_drop_late()  -> Self { Self::new_impl(SlotType::DenseDropLate) }

    // TODO: set_new, \*_ref?, ...

    /// Get the data for this slot associated with `hwnd`.
    ///
    /// ### Errors
    /// *   [ERROR::INVALID_WINDOW_HANDLE]      if `hwnd` is invalid (or, for "early" slots, is being destroyed)
    /// *   [ERROR::WINDOW_OF_OTHER_THREAD]     if `hwnd` belongs to another thread or process
    /// *   [ERROR::DATATYPE_MISMATCH]          if the slot isn't a `T` (bug?)
    ///
    /// ### Example
    /// ```
    /// # use hwnd::*;
    /// # use winresult::*;
    /// # use std::ptr::*;
    /// # let local_process_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// # std::thread::spawn(move ||{
    /// # let local_thread_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// #
    /// use hwnd::assoc::local::*;
    ///
    /// static SLOT : Slot<&'static str> = Slot::new_drop_late();
    ///
    /// # #[cfg(xxx)] {
    /// let local_thread_hwnd   : HWnd = ..; // hwnd belonging to local thread
    /// let local_process_hwnd  : HWnd = ..; // hwnd belonging to local process, another thread
    /// # }
    /// let remote_process_hwnd : HWnd = get_desktop_window();
    ///
    /// # assert_eq!(None,                        SLOT.set(local_thread_hwnd, "slot" ).unwrap());
    /// assert_eq!(Some("slot"),                  SLOT.get_copy(local_thread_hwnd    ).unwrap());
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_copy(local_process_hwnd   ).unwrap_err());
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_copy(remote_process_hwnd  ).unwrap_err());
    /// assert_eq!(ERROR::INVALID_WINDOW_HANDLE,  SLOT.get_copy(HWnd::NULL           ).unwrap_err());
    /// #
    /// # }).join().unwrap();
    /// ```
    pub fn get_copy(&'static self, hwnd: HWnd) -> Result<Option<T>, Error> where T : Copy { self.get_clone(hwnd) }

    /// Get (and [Clone::clone]) the data for this slot associated with `hwnd`.
    ///
    /// ### Errors
    /// *   [ERROR::INVALID_WINDOW_HANDLE]      if `hwnd` is invalid (or, for "early" slots, is being destroyed)
    /// *   [ERROR::WINDOW_OF_OTHER_THREAD]     if `hwnd` belongs to another thread or process
    /// *   [ERROR::DATATYPE_MISMATCH]          if the slot isn't a `T` (bug?)
    ///
    /// ### Example
    /// ```
    /// # use hwnd::*;
    /// # use winresult::*;
    /// # use std::ptr::*;
    /// # let local_process_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// # std::thread::spawn(move ||{
    /// # let local_thread_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// #
    /// use hwnd::assoc::local::*;
    ///
    /// static SLOT : Slot<String> = Slot::new_drop_late();
    ///
    /// # #[cfg(xxx)] {
    /// let local_thread_hwnd   : HWnd = ..; // hwnd belonging to local thread
    /// let local_process_hwnd  : HWnd = ..; // hwnd belonging to local process, another thread
    /// # }
    /// let remote_process_hwnd : HWnd = get_desktop_window();
    ///
    /// # assert_eq!(None,                        SLOT.set(local_thread_hwnd, String::from("slot")).unwrap());
    /// assert_eq!("slot", SLOT.get_clone(local_thread_hwnd).unwrap().unwrap_or(String::new()));
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_clone(local_process_hwnd  ).unwrap_err());
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.get_clone(remote_process_hwnd ).unwrap_err());
    /// assert_eq!(ERROR::INVALID_WINDOW_HANDLE,  SLOT.get_clone(HWnd::NULL          ).unwrap_err());
    /// #
    /// # }).join().unwrap();
    /// ```
    pub fn get_clone(&'static self, hwnd: HWnd) -> Result<Option<T>, Error> where T : Clone {
        check_window_thread_local(hwnd)?;

        let slot_idx = self.slot_idx();
        ThreadLocal::with(move |tl| {
            let forbid_destroying = tl.global.any_early.load(Acquire) && tl.global.dense_slots.read().unwrap()[slot_idx].drop_early;

            let pw = tl.per_window.borrow();
            let pw = match pw.get(&hwnd) {
                None        => return Ok(None),
                Some(pw)    => pw,
            };

            if forbid_destroying && pw.destroying.get() { return fn_err!(ERROR::INVALID_WINDOW_HANDLE) }

            let pw_dense_slots = pw.dense_slots.borrow();
            match pw_dense_slots.get(slot_idx) {
                None                => Ok(None), // slot_idx >= dense_slots.len()
                Some(None)          => Ok(None), // dense_slots[slot_idx].is_none()
                Some(Some(slot))    => {
                    if let Some(slot_t) = slot.downcast_ref::<T>() {
                        Ok(Some((*slot_t).clone()))
                    } else {
                        fn_err!(ERROR::DATATYPE_MISMATCH)
                        // Considered these error codes:
                        //  ERROR::DATATYPE_MISMATCH    Chosen!
                        //  ERROR::INVALID_HANDLE       Kinda makes sense?  Less specific tho.
                        //  ERROR::INVALID_DATATYPE     nah: The datatype is valid, just mismatched?
                        //  ERROR::BAD_TOKEN_TYPE       nah: The token *category* (dense thread local early/late slot) is fine, just what the token references is bad
                        //  ERROR::UNSUPPORTED_TYPE     nah: The type is supported, there's just a mismatch
                    }
                }
            }
        })
    }

    /// Get the data for this slot associated with `hwnd`.
    ///
    /// ### Returns
    /// *   Ok(Some(previous_value))    if the `hwnd` previously had a value for this slot
    /// *   Ok(None)                    if the `hwnd` had no previous value for this slot
    ///
    /// ### Errors
    /// *   [ERROR::INVALID_WINDOW_HANDLE]      if `hwnd` is invalid (or, for "early" slots, is being destroyed)
    /// *   [ERROR::WINDOW_OF_OTHER_THREAD]     if `hwnd` belongs to another thread or process
    /// *   [ERROR::DATATYPE_MISMATCH]          if the previous value in the slot isn't a `T` (bug?)
    ///
    /// ### Example
    /// ```
    /// # use hwnd::*;
    /// # use winresult::*;
    /// # use std::ptr::*;
    /// # let local_process_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// # std::thread::spawn(move ||{
    /// # let local_thread_hwnd = unsafe { create_window_ex_w(0, abistr::utf16ish!("Message"), (), 0, 0, 0, 0, 0, HWnd::MESSAGE, null_mut(), None, null_mut()) }.unwrap();
    /// #
    /// use hwnd::assoc::local::*;
    ///
    /// static SLOT : Slot<&'static str> = Slot::new_drop_late();
    ///
    /// # #[cfg(xxx)] {
    /// let local_thread_hwnd   : HWnd = ..; // hwnd belonging to local thread
    /// let local_process_hwnd  : HWnd = ..; // hwnd belonging to local process, another thread
    /// # }
    /// let remote_process_hwnd : HWnd = get_desktop_window();
    ///
    /// assert_eq!(None,                          SLOT.set(local_thread_hwnd, "init").unwrap());
    /// assert_eq!(Some("init"),                  SLOT.set(local_thread_hwnd, "slot").unwrap());
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.set(local_process_hwnd,   "X").unwrap_err());
    /// assert_eq!(ERROR::WINDOW_OF_OTHER_THREAD, SLOT.set(remote_process_hwnd,  "X").unwrap_err());
    /// assert_eq!(ERROR::INVALID_WINDOW_HANDLE,  SLOT.set(HWnd::NULL,           "X").unwrap_err());
    /// # assert_eq!(Some("slot"),                SLOT.get_copy(local_thread_hwnd    ).unwrap());
    /// #
    /// # }).join().unwrap();
    /// ```
    pub fn set(&'static self, hwnd: HWnd, value: T) -> Result<Option<T>, Error> {
        check_window_thread_local(hwnd)?;
        let value = Box::new(value);

        let slot_idx = self.slot_idx();
        let prev = ThreadLocal::with(move |tl| {
            let forbid_destroying = tl.global.any_early.load(Acquire) && tl.global.dense_slots.read().unwrap()[slot_idx].drop_early;

            let mut pw = tl.per_window.borrow_mut();
            let pw = pw.entry(hwnd).or_default();

            if forbid_destroying && pw.destroying.get() { return fn_err!(ERROR::INVALID_WINDOW_HANDLE) }

            let mut pw_dense_slots = pw.dense_slots.borrow_mut();
            if slot_idx >= pw_dense_slots.len() { pw_dense_slots.resize_with(slot_idx+1, || None) }
            let pw_dense_slot = &mut pw_dense_slots[slot_idx];

            Ok(pw_dense_slot.replace(value))
        })?;

        match prev.map(|p| p.downcast::<T>()) {
            Some(Ok(prev))  => Ok(Some(*prev)),
            Some(Err(_))    => fn_err!(ERROR::DATATYPE_MISMATCH),
            None            => Ok(None),
        }
    }

    const fn new_impl(ty: SlotType) -> Self {
        Self {
            ty,
            dense_slot_no:  AtomicUsize::new(0),
            pd:             PhantomData,
        }
    }

    fn slot_idx(&'static self) -> usize { self.slot_no() - 1 }
    fn slot_no(&'static self) -> usize {
        // https://en.wikipedia.org/wiki/Double-checked_locking

        let s = self.dense_slot_no.load(Acquire);
        if s != 0 { return s }

        let g = Global::get();
        let mut g_dense_slots = g.dense_slots.write().unwrap(); // XXX: avoid panicing?
        let s = self.dense_slot_no.load(Relaxed);
        if s != 0 { return s }

        let drop_early = match self.ty {
            SlotType::DenseDropEarly    => true,
            SlotType::DenseDropLate     => false,
        };

        if drop_early   { g.any_early.store(true, Release); }
        else            { g.any_late .store(true, Release); }

        g_dense_slots.push(DenseSlotMeta { drop_early });
        let slot_no = g_dense_slots.len();
        self.dense_slot_no.store(slot_no, Release);
        slot_no
    }
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] enum SlotType {
    DenseDropEarly,
    DenseDropLate,
}



struct ThreadLocal {
    global:     &'static Global,
    hooks:      Hooks,
    per_window: RefCell<HashMap<HWnd, PerWindow>>,
}

impl Default for ThreadLocal {
    fn default() -> Self {
        Self {
            global:     Global::get(),
            hooks:      Default::default(),
            per_window: Default::default(),
        }
    }
}

impl ThreadLocal {
    fn with<R>(f: impl FnOnce(&ThreadLocal) -> R) -> R {
        thread_local! { static TL : ThreadLocal = ThreadLocal::default(); }
        TL.with(f)
    }
}



#[derive(Default)]
struct Global {
    any_early:      AtomicBool,
    any_late:       AtomicBool,
    dense_slots:    RwLock<Vec<DenseSlotMeta>>,
}

impl Global {
    fn get() -> &'static Self {
        lazy_static::lazy_static! { static ref G : Global = Global::default(); }
        &*G
    }
}



struct DenseSlotMeta {
    drop_early: bool,
}



#[derive(Default)]
struct PerWindow {
    destroying:     Cell<bool>,
    dense_slots:    RefCell<Vec<Option<Box<dyn Any + 'static>>>>,
}

impl PerWindow {
    /// Called before [WM::GETMINMAXINFO] if data is associated with the [HWnd]
    fn before_wm_get_min_max_info(&self) {
        // slot/handle was reused
        self.destroying.set(false);
    }

    /// Called before [WM::NC_CREATE] if data is associated with the [HWnd]
    fn before_wm_nc_create(&self) {
        // slot/handle was reused
        self.destroying.set(false);
    }

    /// Called after [WM::CREATE] if data is associated with the [HWnd]
    fn after_wm_create(&self) {
    }

    /// Called before [WM::DESTROY] if data is associated with the [HWnd]
    fn before_wm_destroy(&mut self, global: &Global) {
        self.destroying.set(true);
        if !global.any_early.load(Acquire) { return }

        // XXX: abort to avoid unwinding through FFI?
        let     g_dense_slots = global.dense_slots.read().unwrap();
        let mut s_dense_slots = self.dense_slots.borrow_mut();

        for (g_dense, s_dense) in g_dense_slots.iter().zip(s_dense_slots.iter_mut()) {
            if g_dense.drop_early {
                // TODO: abort on panics?  Avoids:
                //  * unwinding across FFI boundaries
                //  * early Drop s outliving WM::DESTROY
                let _ = s_dense.take();
            }
        }
    }

    /// Called after [WM::NCDESTROY] if data is associated with the [HWnd]
    fn after_wm_nc_destroy(mut self) {
        // TODO: control drop order?
        self.dense_slots.get_mut().clear();
    }
}



struct Hooks {
    wh_cbt:             HHOOK,
    wh_callwndproc:     HHOOK,
    wh_callwndprocret:  HHOOK,
}

impl Default for Hooks {
    fn default() -> Self {
        let thread = get_current_thread_id();
        let hooks = Self {
            wh_cbt:             unsafe { SetWindowsHookExW(WH_CBT,              Some(wh_cbt),               null_mut(), thread) },
            wh_callwndproc:     unsafe { SetWindowsHookExW(WH_CALLWNDPROC,      Some(wh_callwndproc),       null_mut(), thread) },
            wh_callwndprocret:  unsafe { SetWindowsHookExW(WH_CALLWNDPROCRET,   Some(wh_callwndprocret),    null_mut(), thread) },
        };
        debug_assert!(!hooks.wh_cbt             .is_null());
        debug_assert!(!hooks.wh_callwndproc     .is_null());
        debug_assert!(!hooks.wh_callwndprocret  .is_null());
        hooks
    }
}

impl Drop for Hooks {
    fn drop(&mut self) {
        let unhook_wh_cbt               = self.wh_cbt.is_null()             || unsafe { UnhookWindowsHookEx(self.wh_cbt             ) != 0 };
        let unhook_wh_callwndproc       = self.wh_callwndproc.is_null()     || unsafe { UnhookWindowsHookEx(self.wh_callwndproc     ) != 0 };
        let unhook_wh_callwndprocret    = self.wh_callwndprocret.is_null()  || unsafe { UnhookWindowsHookEx(self.wh_callwndprocret  ) != 0 };
        debug_assert!(unhook_wh_cbt);
        debug_assert!(unhook_wh_callwndproc);
        debug_assert!(unhook_wh_callwndprocret);
    }
}

/// Require [get_current_thread_id]\(\) == [get_window_thread_id]\(hwnd\)
fn check_window_thread_local(hwnd: HWnd) -> Result<(), Error> {
    fn_context!(assoc::local::check_window_thread_local => GetWindowThreadProcessId);
    let tid = get_window_thread_id(hwnd)?;
    if get_current_thread_id() != tid { return fn_err!(ERROR::WINDOW_OF_OTHER_THREAD) }
    Ok(())
}

// TODO: catch panics to avoid unwinding across FFI boundaries?
// Rust plans to catch unwinding through extern "system", but that hasn't landed yet:
// https://github.com/rust-lang/rust/issues/52652
// Review/use: https://docs.rs/unwind_aborts/ ?

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644977(v=vs.85)) \]
unsafe extern "system" fn wh_cbt(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let hook = ThreadLocal::with(|tl| tl.hooks.wh_cbt);

    if code >= 0 {
    }

    let lr = unsafe { CallNextHookEx(hook, code, wparam, lparam) };
    lr
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644975(v=vs.85)) \]
/// CallWndProc callback
unsafe extern "system" fn wh_callwndproc(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let hook = ThreadLocal::with(|tl| tl.hooks.wh_callwndproc);

    if code == HC_ACTION {
        // "Specifies whether the message was sent by the current thread. If the message was sent by the current thread, it is nonzero; otherwise, it is zero."
        // NOTE: CallWndRetProc thinks this instead indicates if the message is instead sent by the current *process*.
        let _from_current_proc_or_thread = wparam != 0;

        let call    = unsafe { &*(lparam as *const CWPSTRUCT) };
        let hwnd    = HWnd::from(call.hwnd);
        let msg     = WM32::from(call.message);

        if !call.hwnd.is_null() {
            match msg {
                WM::GETMINMAXINFO   => { ThreadLocal::with(|tl| tl.per_window.borrow    ().get    (&hwnd).map(|pw| pw.before_wm_get_min_max_info()  )); },
                WM::NCCREATE        => { ThreadLocal::with(|tl| tl.per_window.borrow    ().get    (&hwnd).map(|pw| pw.before_wm_nc_create()         )); },
                WM::DESTROY         => { ThreadLocal::with(|tl| tl.per_window.borrow_mut().get_mut(&hwnd).map(|pw| pw.before_wm_destroy(tl.global)  )); },
                _                   => {}
            }
        }
    }

    let lr = unsafe { CallNextHookEx(hook, code, wparam, lparam) };
    lr
}

/// \[[learn.microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nc-winuser-hookproc)\]
/// CallWndRetProc
unsafe extern "system" fn wh_callwndprocret(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let hook = ThreadLocal::with(|tl| tl.hooks.wh_callwndprocret);

    if code >= 0 {
        // "Specifies whether the message is sent by the current process. If the message is sent by the current process, it is nonzero; otherwise, it is NULL."
        // NOTE: CallWndProc thinks this instead indicates if the message is instead sent by the current *thread*.
        let _from_current_proc_or_thread = wparam != 0;

        let ret     = unsafe { &*(lparam as *const CWPRETSTRUCT) };
        let hwnd    = HWnd::from(ret.hwnd);
        let msg     = WM32::from(ret.message);

        if !hwnd.is_null() {
            match msg {
                WM::CREATE      => { ThreadLocal::with(|tl| tl.per_window.borrow    ().get   (&hwnd).map(|pw| pw.after_wm_create()    )); }
                WM::NCDESTROY   => { ThreadLocal::with(|tl| tl.per_window.borrow_mut().remove(&hwnd)).map(|pw| pw.after_wm_nc_destroy()); }
                _               => {}
            }
        }
    }

    let lr = unsafe { CallNextHookEx(hook, code, wparam, lparam) };
    lr
}
