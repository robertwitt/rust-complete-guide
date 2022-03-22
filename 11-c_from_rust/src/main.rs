use std::ffi::CString;
use std::os::raw::{c_char, c_uint};

extern "C" {
    fn mystrlen(str: *const c_char) -> c_uint;
}

fn safe_mystrlen(str: &str) -> Option<u32> {
    let c_string = match CString::new(str) {
        Ok(c) => c,
        Err(_) => return None,
    };
    unsafe { Some(mystrlen(c_string.as_ptr())) }
}

fn main() {
    match safe_mystrlen("C From Rust") {
        Some(count) => println!("c_string's length is {}", count),
        None => println!("Not a C string"),
    }
}
