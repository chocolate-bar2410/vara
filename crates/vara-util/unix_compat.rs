use termios::{cfmakeraw,tcsetattr,Termios,TCSANOW};
use mlua::ExternalResult;
use std::sync::Mutex;

static ORIGINAL_TERM_MODE: Mutex<Option<Termios>> = Mutex::new(None);

pub fn enter_raw_mode() -> mlua::Result<()> {
    let original = Termios::from_fd(0).into_lua_err()?;
    let mut raw = original;

    cfmakeraw(&mut raw);
    tcsetattr(0,TCSANOW, &raw).into_lua_err()?;

    let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| mlua::Error::external(format!("terminal original mode mutex poisioned {err}")))?;

    *original_mutex = Some(original);

    Ok(())
}

pub fn exit_raw_mode() -> mlua::Result<()> {
    let mut original_mutex = ORIGINAL_TERM_MODE.lock()
            .map_err(|err| mlua::Error::external(format!("terminal original mode mutex poisioned {err}")))?;

    if let Some(original) = original_mutex.take() {
        tcsetattr(0,TCSANOW, &original).into_lua_err()?;
    }

    Ok(())
}