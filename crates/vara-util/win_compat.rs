// if only windows used utf8 🥀
use std::ffi::c_uint;
const CP_UTF8: c_uint = 65001;

#[repr(C)]
struct TimeZoneInformation {
    bias: i32,
    _rest: [u8; 164], // ignore remaining fields
    daylight_bias: i32,
}

unsafe extern "system" {
    fn SetConsoleOutputCP(wCodePageID: c_uint) -> i32;
    fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TimeZoneInformation) -> u32;
}

pub fn init() {
    unsafe { SetConsoleOutputCP(CP_UTF8) };
}

pub fn get_timezone_offset() -> i32 {
    unsafe {
        let mut tz = std::mem::zeroed::<TimeZoneInformation>();
        let result = GetTimeZoneInformation(&mut tz);

        let bias = if result == 2 {
            tz.bias + tz.daylight_bias
        } else {
            tz.bias
        };

        // Windows bias is minutes west of UTC
        -bias * 60 
    }
}
