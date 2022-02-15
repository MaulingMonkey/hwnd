use crate::{*, WNDCLASSA, WNDCLASSW, WNDCLASSEXA, WNDCLASSEXW};
use winapi::um::winuser::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassa)\]
/// RegisterClassA
///
/// Registers a window class for subsequent use in calls to the CreateWindow or CreateWindowEx function.
///
/// ### Safety
/// Code will make various implicit and undocumented assumptions about a given registered `class_name`.
/// This might include things such as the type referenced by userdata, creation parameters, expected WM_USER+N message formats, etc.
/// It is unsound to violate any of those assumptions.
///
/// Additionally:
/// *   `class.wnd_proc`    must be well behaved
/// *   `class.h*` handles  must be null or sane (static?)
/// *   `class.*_name`      must be None or sane
///
/// ### Errors
/// *   [ERROR::CLASS_ALREADY_EXISTS]   If `class_name` was already registered
/// *   [ERROR::INVALID_PARAMETER]      If no `class_name` is specified
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// let wndclass = WNDCLASSA {
///     class_name: Some(abistr::cstr!("my_narrow_class")),
///     .. Default::default()
/// };
///
/// unsafe { register_class_a(&wndclass) }.unwrap();
/// #
/// # assert_eq!(Some(ERROR::CLASS_ALREADY_EXISTS), unsafe { register_class_a(&wndclass) }.unwrap_err().code());
/// # assert_eq!(Some(ERROR::INVALID_PARAMETER), unsafe { register_class_a(&WNDCLASSA::default()) }.unwrap_err().code());
/// ```
pub unsafe fn register_class_a(class: &WNDCLASSA) -> Result<Atom, Error> {
    fn_context!(register_class_a => RegisterClassA);
    let atom = unsafe { RegisterClassA(class.as_ref()) };
    fn_succeeded!(atom)?;
    Ok(Atom(atom))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassw)\]
/// RegisterClassW
///
/// Registers a window class for subsequent use in calls to the CreateWindow or CreateWindowEx function.
///
/// ### Safety
/// Code will make various implicit and undocumented assumptions about a given registered `class_name`.
/// This might include things such as the type referenced by userdata, creation parameters, expected WM_USER+N message formats, etc.
/// It is unsound to violate any of those assumptions.
///
/// Additionally:
/// *   `class.wnd_proc`    must be well behaved
/// *   `class.h*` handles  must be null or sane (static?)
/// *   `class.*_name`      must be None or sane
///
/// ### Errors
/// *   [ERROR::CLASS_ALREADY_EXISTS]   If `class_name` was already registered
/// *   [ERROR::INVALID_PARAMETER]      If no `class_name` is specified
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// let wndclass = WNDCLASSW {
///     class_name: Some(abistr::cstr16!("my_unicode_class")),
///     .. Default::default()
/// };
///
/// unsafe { register_class_w(&wndclass) }.unwrap();
/// #
/// # assert_eq!(Some(ERROR::CLASS_ALREADY_EXISTS), unsafe { register_class_w(&wndclass) }.unwrap_err().code());
/// # assert_eq!(Some(ERROR::INVALID_PARAMETER), unsafe { register_class_w(&WNDCLASSW::default()) }.unwrap_err().code());
/// ```
pub unsafe fn register_class_w(class: &WNDCLASSW) -> Result<Atom, Error> {
    fn_context!(register_class_w => RegisterClassW);
    let atom = unsafe { RegisterClassW(class.as_ref()) };
    fn_succeeded!(atom)?;
    Ok(Atom(atom))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexa)\]
/// RegisterClassExA
///
/// Registers a window class for subsequent use in calls to the CreateWindow or CreateWindowEx function.
///
/// ### Safety
/// Code will make various implicit and undocumented assumptions about a given registered `class_name`.
/// This might include things such as the type referenced by userdata, creation parameters, expected WM_USER+N message formats, etc.
/// It is unsound to violate any of those assumptions.
///
/// Additionally:
/// *   `class.wnd_proc`    must be well behaved
/// *   `class.h*` handles  must be null or sane (static?)
/// *   `class.*_name`      must be None or sane
///
/// ### Errors
/// *   [ERROR::CLASS_ALREADY_EXISTS]   If `class_name` was already registered
/// *   [ERROR::INVALID_PARAMETER]      If no `class_name` is specified
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// let wndclass = WNDCLASSEXA {
///     class_name: Some(abistr::cstr!("my_narrow_class_ex")),
///     .. Default::default()
/// };
/// dbg!(wndclass.size);
///
/// unsafe { register_class_ex_a(&wndclass) }.unwrap();
/// #
/// # assert_eq!(Some(ERROR::CLASS_ALREADY_EXISTS), unsafe { register_class_ex_a(&wndclass) }.unwrap_err().code());
/// # assert_eq!(Some(ERROR::INVALID_PARAMETER), unsafe { register_class_ex_a(&WNDCLASSEXA::default()) }.unwrap_err().code());
/// ```
pub unsafe fn register_class_ex_a(class: &WNDCLASSEXA) -> Result<Atom, Error> {
    fn_context!(register_class_ex_a => RegisterClassExA);
    if class.size != size_of_32::<WNDCLASSEXW>() { Err(fn_param_error!(class.size, ERROR::INVALID_PARAMETER))? }
    let atom = unsafe { RegisterClassExA(class.as_ref()) };
    fn_succeeded!(atom)?;
    Ok(Atom(atom))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-registerclassexw)\]
/// RegisterClassExW
///
/// Registers a window class for subsequent use in calls to the CreateWindow or CreateWindowEx function.
///
/// ### Safety
/// Code will make various implicit and undocumented assumptions about a given registered `class_name`.
/// This might include things such as the type referenced by userdata, creation parameters, expected WM_USER+N message formats, etc.
/// It is unsound to violate any of those assumptions.
///
/// Additionally:
/// *   `class.wnd_proc`    must be well behaved
/// *   `class.h*` handles  must be null or sane (static?)
/// *   `class.*_name`      must be None or sane
///
/// ### Errors
/// *   [ERROR::CLASS_ALREADY_EXISTS]   If `class_name` was already registered
/// *   [ERROR::INVALID_PARAMETER]      If no `class_name` is specified
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// let wndclass = WNDCLASSEXW {
///     class_name: Some(abistr::cstr16!("my_unicode_class_ex")),
///     .. Default::default()
/// };
///
/// unsafe { register_class_ex_w(&wndclass) }.unwrap();
/// #
/// # assert_eq!(Some(ERROR::CLASS_ALREADY_EXISTS), unsafe { register_class_ex_w(&wndclass) }.unwrap_err().code());
/// # assert_eq!(Some(ERROR::INVALID_PARAMETER), unsafe { register_class_ex_w(&WNDCLASSEXW::default()) }.unwrap_err().code());
/// ```
pub unsafe fn register_class_ex_w(class: &WNDCLASSEXW) -> Result<Atom, Error> {
    fn_context!(register_class_ex_w => RegisterClassExW);
    if class.size != size_of_32::<WNDCLASSEXW>() { Err(fn_param_error!(class.size, ERROR::INVALID_PARAMETER))? }
    let atom = unsafe { RegisterClassExW(class.as_ref()) };
    fn_succeeded!(atom)?;
    Ok(Atom(atom))
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassa)\]
/// UnregisterClassA
///
/// Unregisters a window class, freeing the memory required for the class.
///
/// ### Safety
/// If you unregister an expected class:
/// *   C++ code is likely to exhibit undefined behavior.
/// *   `unsafe`-spamming rust code is likely to exhibit undefined behavior.
/// *   `hwnd`-based rust code may panic, but "should" be safe.  Probably.
///
/// ### Errors
/// *   [ERROR::CLASS_DOES_NOT_EXIST]   If `class_name` is a string, and does not exist for `hinstance`.
/// *   [ERROR::INVALID_HANDLE]         If `class_name` is an atom, and does not exist for `hinstance`.
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// #
/// let class_name = abistr::cstr!("my_narrow_temp_class");
/// let wndclass = WNDCLASSA {
///     class_name: Some(class_name),
///     .. Default::default()
/// };
/// unsafe {
///
///     // Register / unregister by class name
///     register_class_a(&wndclass).unwrap();
///     unregister_class_a(class_name, null_mut()).unwrap();
///     assert_eq!(
///         Some(ERROR::CLASS_DOES_NOT_EXIST),
///         unregister_class_a(class_name, null_mut()).unwrap_err().code(),
///     );
///
///
///     // Register by class name, unregister by atom
///     let atom = register_class_a(&wndclass).unwrap();
///     unregister_class_a(atom, null_mut()).unwrap();
///     assert_eq!(
///         Some(ERROR::INVALID_HANDLE),
///         unregister_class_a(atom, null_mut()).unwrap_err().code(),
///     );
///
///     # assert_eq!(Some(ERROR::CLASS_DOES_NOT_EXIST), unregister_class_a(class_name, 12 as _).unwrap_err().code());
///     # assert_eq!(Some(ERROR::CLASS_DOES_NOT_EXIST), unregister_class_a(class_name, !42usize as _).unwrap_err().code());
/// }
/// ```
// TODO: test unregistering a class that has active windows
pub unsafe fn unregister_class_a<'c>(class_name: impl Into<NameAtomOrZero<'c, u8>>, hinstance: HINSTANCE) -> Result<(), Error> {
    fn_context!(unregister_class_a => UnregisterClassA);
    fn_succeeded!(unsafe { UnregisterClassA(class_name.into().as_atom_or_cstr_ptr(), hinstance) })
}

/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-unregisterclassw)\]
/// UnregisterClassW
///
/// Unregisters a window class, freeing the memory required for the class.
///
/// ### Safety
/// If you unregister an expected class:
/// *   C++ code is likely to exhibit undefined behavior.
/// *   `unsafe`-spamming rust code is likely to exhibit undefined behavior.
/// *   `hwnd`-based rust code may panic, but "should" be safe.  Probably.
///
/// ### Errors
/// *   [ERROR::CLASS_DOES_NOT_EXIST]   If `class_name` is a string, and does not exist for `hinstance`.
/// *   [ERROR::INVALID_HANDLE]         If `class_name` is an atom, and does not exist for `hinstance`.
///
/// ### Example
/// ```
/// # use hwnd::*;
/// # use winerr::*;
/// # use std::ptr::*;
/// #
/// let class_name = abistr::cstr16!("my_unicode_temp_class");
/// let wndclass = WNDCLASSW {
///     class_name: Some(class_name),
///     .. Default::default()
/// };
/// unsafe {
///
///     // Register / unregister by class name
///     register_class_w(&wndclass).unwrap();
///     unregister_class_w(class_name, null_mut()).unwrap();
///     assert_eq!(
///         Some(ERROR::CLASS_DOES_NOT_EXIST),
///         unregister_class_w(class_name, null_mut()).unwrap_err().code(),
///     );
///
///
///     // Register by class name, unregister by atom
///     let atom = register_class_w(&wndclass).unwrap();
///     unregister_class_w(atom, null_mut()).unwrap();
///     assert_eq!(
///         Some(ERROR::INVALID_HANDLE),
///         unregister_class_w(atom, null_mut()).unwrap_err().code(),
///     );
///
///     # assert_eq!(Some(ERROR::CLASS_DOES_NOT_EXIST), unregister_class_w(class_name, 12 as _).unwrap_err().code());
///     # assert_eq!(Some(ERROR::CLASS_DOES_NOT_EXIST), unregister_class_w(class_name, !42usize as _).unwrap_err().code());
/// }
/// ```
// TODO: test unregistering a class that has active windows
pub unsafe fn unregister_class_w<'c>(class_name: impl Into<NameAtomOrZero<'c, u16>>, hinstance: HINSTANCE) -> Result<(), Error> {
    fn_context!(unregister_class_w => UnregisterClassW);
    fn_succeeded!(unsafe { UnregisterClassW(class_name.into().as_atom_or_cstr_ptr(), hinstance) })
}
