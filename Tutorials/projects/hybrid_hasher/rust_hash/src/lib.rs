use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use sha2::{Sha256, Digest};

#[no_mangle]
pub extern "C" fn rust_hash_string(input: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(input) };
    let input_str = c_str.to_str().unwrap();

    let mut hasher = Sha256::new();
    hasher.update(input_str);
    let result = hasher.finalize();
    let hex = result.iter().map(|b| format!("{:02x}", b)).collect::<String>();

    CString::new(hex).unwrap().into_raw()
}
