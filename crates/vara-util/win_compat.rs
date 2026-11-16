use std::sync::Mutex;

use mlua::{ExternalError, ExternalResult};

use windows::Win32::System::{
    Console::{
        SetConsoleOutputCP,
        STD_INPUT_HANDLE,
        ENABLE_LINE_INPUT,
        ENABLE_ECHO_INPUT,
        ENABLE_PROCESSED_INPUT,
        ENABLE_VIRTUAL_TERMINAL_INPUT,
        CONSOLE_MODE,
        GetStdHandle,
        GetConsoleMode,
        SetConsoleMode,
    },
    Time::{
        TIME_ZONE_INFORMATION,
        GetTimeZoneInformation
    }
};

const CP_UTF8: u32 = 65001;
// teriminal

static ORIGINAL_TERM_MODE: Mutex<Option<CONSOLE_MODE>> = Mutex::new(None);

// if only windows used utf8 🥀
pub fn init() -> Result<(),Box<dyn std::error::Error>> {
    unsafe { SetConsoleOutputCP(CP_UTF8)? };

    Ok(())
}

pub fn get_timezone_offset() -> i32 {
    unsafe {
        let mut tz = std::mem::zeroed::<TIME_ZONE_INFORMATION>();
        let result = GetTimeZoneInformation(&mut tz);

        let bias = if result == 2 {
            tz.Bias + tz.DaylightBias
        } else {
            tz.Bias
        };

        // Windows bias is minutes west of UTC
        -bias * 60 
    }
}

pub fn enter_raw_mode() -> mlua::Result<()> {
    unsafe {
        let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| format!("terminal original mode mutex poisioned {err}")
            .into_lua_err())?;

        let handle = GetStdHandle(STD_INPUT_HANDLE).into_lua_err()?;
        let mut mode = CONSOLE_MODE::default();

        GetConsoleMode(handle, &mut mode).into_lua_err()?;

        let original = mode;

        mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);
        mode |= ENABLE_VIRTUAL_TERMINAL_INPUT;

        SetConsoleMode(handle, mode).into_lua_err()?;

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
            let handle = GetStdHandle(STD_INPUT_HANDLE).into_lua_err()?;
            
            SetConsoleMode(handle, original).into_lua_err()?;
        }

        Ok(())
    }
}
