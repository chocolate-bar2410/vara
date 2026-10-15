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

pub fn all(context: &mlua::Lua,tasks : mlua::MultiValue) -> mlua::Result<()>{ 
    if tasks.is_empty() {return Ok(())}
    let mut threads : Vec<mlua::Thread> = Vec::new();

    for(i,value) in tasks.into_iter().enumerate() {
        
        if let Some(thread) = value.as_thread() {
            threads.push(thread.clone());

            continue;
        };

        let Some(lua_fn) = value.as_function() else {
            return Err(format!("Argument at position {} is not a function",i + 1).into_lua_err());
        };

        let thread = context.create_thread(lua_fn.clone())?;

        threads.push(thread);
    }

    if threads.is_empty() {return Ok(())}

    let mut finished = false;

    while finished == false {
        for thread in &threads {
            if thread.is_finished() {continue}

            thread.resume::<()>(())?;
        }

        finished = true;
        for thread in &threads {
            finished = thread.is_finished() && finished;
        }
    }

    Ok(())
}

pub fn race(context: &mlua::Lua,tasks : mlua::MultiValue) -> mlua::Result<mlua::MultiValue>{ 
    if tasks.is_empty() {return Ok(mlua::MultiValue::new())}

    let mut threads : Vec<mlua::Thread> = Vec::new();

    for(i,value) in tasks.into_iter().enumerate() {
        
        if let Some(thread) = value.as_thread() {
            if thread.is_finished() {continue}

            threads.push(thread.clone());

            continue;
        };

        let Some(lua_fn) = value.as_function() else {
            return Err(format!("Argument at position {} is not a function",i + 1).into_lua_err());
        };

        let thread = context.create_thread(lua_fn.clone())?;

        threads.push(thread);
    }

    if threads.is_empty() {return Ok(mlua::MultiValue::new())}

    loop {
        for thread in &threads {
            let result = thread.resume::<mlua::MultiValue>(())?;

            if !thread.is_finished() {continue}

            return Ok(result);
        }
    }
}



pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {

    let async_fns = Functions::new(context.clone())?;

    TableBuilder::new(context.clone())?
        .add_async_function("Sleep", sleep)?
        .add_value("Spawn", async_fns.spawn)?
        .add_value("Defer", async_fns.defer)?
        .add_value("Cancel", async_fns.cancel)?
        .add_function("All", all)?
        .add_function("Race", race)?
        .build_readonly()
}
