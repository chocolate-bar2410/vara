use smol::Timer;
use std::time::Duration;

use vara_util::table_builder::TableBuilder;

pub async fn sleep(_: mlua::Lua,duration : f64) -> mlua::Result<()>{    
    Timer::after(Duration::from_secs_f64(duration)).await;
    Ok(())
}

pub async fn pool(context: mlua::Lua,workers : u32) -> mlua::Result<()>{    

    Ok(())
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    TableBuilder::new(context.clone())?
        .add_async_function("Sleep", sleep)?
        .build_readonly()
}
