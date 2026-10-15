use mlua::ExternalError;
use smol::Timer;
use std::time::Duration;

use vara_util::table_builder::TableBuilder;

use mlua_luau_scheduler::Functions;

pub async fn sleep(_: mlua::Lua,duration : Option<f64>) -> mlua::Result<()>{    

    let time = Duration::from_secs_f64(duration.unwrap_or_default()).max(Duration::from_millis(1)); 

    Timer::after(time).await;

    Ok(())
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {

    let async_fns = Functions::new(context.clone())?;

    TableBuilder::new(context.clone())?
        .add_async_function("Sleep", sleep)?
        .add_value("Spawn", async_fns.spawn)?
        .add_value("Defer", async_fns.defer)?
        .add_value("Cancel", async_fns.cancel)?
        .build_readonly()
}
