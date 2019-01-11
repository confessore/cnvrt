use std::ffi::CStr;
use std::os::raw::c_char;

/// converts '*const c_char' to '&str'
pub fn cchar_to_str(cchar: *const c_char) -> &'static str {
    let c_buf: *const c_char = cchar;
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    c_str.to_str().unwrap()
}

/// converts '&[i8]' to 'String'
pub fn utf8_to_string(bytes: &[i8]) -> String {
        unsafe { CStr::from_ptr(bytes.as_ptr()).to_string_lossy().into_owned() }
}

