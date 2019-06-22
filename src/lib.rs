#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

use std::ffi::CString;
use std::os::raw::c_char;

static HELLO: &'static str = "Hello from Rust";

#[no_mangle]
pub fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn get_hello_len() -> usize {
    HELLO.len()
}