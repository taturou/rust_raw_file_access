extern crate libc;
use std::ffi::CString;
use std::os::raw::c_char;
use libc::{c_int, size_t, mode_t};

#[link(name="rawopen", kind="static")]
extern {
    // fn raw_open(pathname: *const c_char, flags: c_int, mode: mode_t) -> c_int;
    fn raw_open(pathname: *const c_char) -> c_int;
    fn raw_close(fd: c_int) -> c_int;
}

fn s_open(pathname: &str) -> i32 {
    unsafe {
        let pathname = CString::new(pathname).unwrap();
        let fd = raw_open(pathname.as_ptr() as *const c_char);
        fd as i32
    }
}

fn s_close(fd: i32) -> Result<i32, i32> {
    unsafe {
        match raw_close(fd as c_int) {
            0 => Ok(0),
            _ => Err(-1)
        }
    }
}

fn main() {
    let fd = s_open("/dev/null");
    println!("s_open() -> {}", fd);
    let ret = s_close(fd);
    println!("s_close() -> {:?}", ret);
}
