//use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::SetLastError;

pub(crate) fn clear_last_error() { set_last_error(0) }
pub(crate) fn set_last_error(code: u32) { unsafe { SetLastError(code) } }

// XXX: To avoid tripping up other code, this should probably be a scoped thing that also clears the error code
//pub(crate) fn debug_set_last_error_invalid() {
//    if cfg!(debug_assertions) {
//        //set_last_error(ERROR_INTERNAL_ERROR);
//        set_last_error(ERROR_INVALID_FUNCTION);
//    }
//}
