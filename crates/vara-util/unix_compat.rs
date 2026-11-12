use std::io;
use termios::{cfmakeraw,tcsetattr,Termios,TSCANOW};
use mlua::ExternalError;

static ORIGINAL_TERM_MODE: Mutex<Option<Termios>> = Mutex::new(None);

pub fn enter_raw_mode() -> mlua::result<()> {
    let original = Termios::from_fd(0);
    let mut raw = original;

    cfmakeraw(&mut raw);
    tcsetattr(0,TCSANOW, &raw).into_lua_err()?;

    let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| format!("terminal original mode mutex poisioned {err}")
            .into_lua_err())?;

    *original_mutex = Some(original);

    Ok(())
}

pub fn exit_raw_mode() -> mlua::result<()> {
    if let Some(original) = original_mutex.take() {
        tcsetattr(0,TCSANOW, original).into_lua_err()?;
    }
}