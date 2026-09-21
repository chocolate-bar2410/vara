use std::env::consts::{ARCH, FAMILY, OS};
use mlua::{ExternalError, IntoLua};
use vara_util::table_builder::TableBuilder;

use std::thread::available_parallelism as cpus_available;

fn get_language_preferences(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let lang_preference: whoami::LanguagePreferences = whoami::lang_prefs().unwrap_or_default();

    let time_langs: Vec<String> = lang_preference
        .time_langs()
        .map(|lang| lang.to_string())
        .collect();

    let monetary_langs: Vec<String> = lang_preference
        .monetary_langs()
        .map(|lang| lang.to_string())
        .collect();

    let collation_langs: Vec<String> = lang_preference
        .collation_langs()
        .map(|lang| lang.to_string())
        .collect();

    let message_langs: Vec<String> = lang_preference
        .message_langs()
        .map(|lang| lang.to_string())
        .collect();

    let numeric_langs: Vec<String> = lang_preference
        .numeric_langs()
        .map(|lang| lang.to_string())
        .collect();

    TableBuilder::new(context.clone())?
        .add_values(vec![
            ("time",context.create_sequence_from(time_langs)?),
            ("monetary",context.create_sequence_from(monetary_langs)?),
            ("collation",context.create_sequence_from(collation_langs)?),
            ("message",context.create_sequence_from(message_langs)?),
            ("numeric",context.create_sequence_from(numeric_langs)?),
            ])?
        .build_readonly()
}

fn get_cpu_count(context : &mlua::Lua,_ : ()) -> mlua::Result<mlua::Value>{
    let count = cpus_available()?.get();

    Ok(count.into_lua(context)?)
}

fn get_physical_memory_usage(context : &mlua::Lua,_ : ()) -> mlua::Result<mlua::Value>{
    let Some(usage) = memory_stats::memory_stats() else {
       return Err("could not get memory usage".into_lua_err());
    };

    Ok(usage.physical_mem.into_lua(context)?)
}


fn get_virtual_memory_usage(context : &mlua::Lua,_ : ()) -> mlua::Result<mlua::Value>{
    let Some(usage) = memory_stats::memory_stats() else {
       return Err("could not get memory usage".into_lua_err());
    };

    Ok(usage.virtual_mem.into_lua(context)?)
}

pub fn get_module(context: &mlua::Lua) -> mlua::Result<mlua::Table> {
    dotenvy::dotenv().map_err(|err| err.into_lua_err())?;
    
    TableBuilder::new(context.clone())?
        .add_value("os", OS)?
        .add_value("arch", ARCH)?
        .add_value("family", FAMILY)?
        .add_value("cwd", std::env::current_dir()?)?
        .add_value("temp", std::env::temp_dir())?
        .add_value(
            "endian",
            if cfg!(target_endian = "big") {
                "big"
            } else {
                "little"
            },
        )?
        .add_value("languagePreferences", get_language_preferences(context)?)?
        .add_value("distro", match whoami::distro(){
            Ok(distro) => distro,
            _ => "unknown".to_string()
        })?
        .add_value("device",match whoami::devicename(){
            Ok(device_name) => device_name,
            _ => "unknown".to_string()
        })?
        .add_value("processID", std::process::id())?
        .add_value("desktopEnvironment", whoami::desktop_env().map(|env| format!("{:?}",env)).unwrap_or_else(|| "Unknown".to_string()))?
        .add_value("env", context.create_table_from(std::env::vars())?)?
        .add_function("processorCount", get_cpu_count)?
        .add_function("physicalMemoryUsage", get_physical_memory_usage)?
        .add_function("virtualMemoryUsage", get_virtual_memory_usage)?
        .build_readonly()
}
