use std::{ffi::{c_uint, c_void}, sync::Mutex};

use mlua::ExternalError;

const CP_UTF8: c_uint = 65001;
// teriminal
const STD_INPUT_HANDLE: u32 = (-10i32) as u32;
const ENABLE_LINE_INPUT: u32 = 0x0002;
const ENABLE_ECHO_INPUT: u32 = 0x0004;
const ENABLE_PROCESSED_INPUT: u32 = 0x0001;

type Handle = *mut c_void;

static ORIGINAL_TERM_MODE: Mutex<Option<u32>> = Mutex::new(None);

#[repr(C)]
struct TimeZoneInformation {
    bias: i32,
    _rest: [u8; 164], // ignore remaining fields
    daylight_bias: i32,
}

unsafe extern "system" {
    fn SetConsoleOutputCP(wCodePageID: c_uint) -> i32;
    fn GetTimeZoneInformation(lpTimeZoneInformation: *mut TimeZoneInformation) -> u32;

    fn GetStdHandle(nStdHandle: u32) -> Handle;
    fn GetConsoleMode(hConsoleMode: Handle, lpMode: *mut u32) -> i32;
    fn SetConsoleMode(hConsoleMode: Handle, dwMode: u32) -> i32;
}

// if only windows used utf8 🥀
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

pub fn enter_raw_mode() -> mlua::Result<()> {
    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        let mut mode = 0u32;

        if GetConsoleMode(handle, &mut mode) == 0 {
            return Err(std::io::Error::last_os_error().into_lua_err())
        }

        let original = mode;

        mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);

        if SetConsoleMode(handle, mode) == 0 {
            return Err(std::io::Error::last_os_error().into_lua_err())
        }

        let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| format!("terminal original mode mutex poisioned {err}")
            .into_lua_err())?;

        *original_mutex = Some(original);

        Ok(())
    }
}

pub fn exit_raw_mode() -> mlua::Result<()>  {
    unsafe {
        let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| format!("terminal original mode mutex poisioned {err}")
            .into_lua_err())?;

        if let Some(original) = original_mutex.take() {
            let handle = GetStdHandle(STD_INPUT_HANDLE);
            
            if SetConsoleMode(handle, original) == 0 {
                return Err(std::io::Error::last_os_error().into_lua_err())
            }
        }

        Ok(())
    }
}
