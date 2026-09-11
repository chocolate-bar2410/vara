// if only windows used utf8 🥀
use std::ffi::c_uint;
const CP_UTF8: c_uint = 65001;

#[cfg(windows)]
unsafe extern "system" {
    fn SetConsoleOutputCP(wCodePageID: c_uint) -> i32;
}

pub fn init() {
    unsafe { SetConsoleOutputCP(CP_UTF8) };
}
