use mlua::{ExternalError, ExternalResult};
use slint_interpreter::{self as slint, ComponentHandle};

fn luau_to_slint(value : mlua::Value) -> Result<slint::Value,mlua::error::Error>{
    match value {
        mlua::Value::Boolean(value) => Ok(slint::Value::Bool(value)),
        mlua::Value::Number(value) => Ok(slint::Value::Number(value)),

        // wtf is an integer, luau doesn't even have integer types :sob:
        mlua::Value::Integer(value) => Ok(slint::Value::Number(value as f64)),
        mlua::Value::String(value) => Ok(slint::Value::String(value.to_string_lossy().into())),
        mlua::Value::Nil => Ok(slint::Value::Void),

        _ => Err(format!("cannot translate luau type to slint type").into_lua_err()),
    }
}

    fn slint_to_luau(context : &mlua::Lua,value : &slint::Value) -> Result<mlua::Value,mlua::error::Error>{
    match value {
        slint::Value::Bool(value) => Ok(mlua::Value::Boolean(*value)),
        slint::Value::Number(value) => Ok(mlua::Value::Number(*value)),
        slint::Value::String(value) => Ok(mlua::Value::String(context.create_string(value.as_str())?)),
        slint::Value::Void => Ok(mlua::Value::Nil),

        _ => Err(format!("cannot translate slint type to luau type").into_lua_err()),
    }
}

pub struct UIInstance{
    instance : slint::ComponentInstance,
}

impl UIInstance{
    pub fn new(defintion : &slint::ComponentDefinition) -> Self {
        Self{
            instance : defintion.create().unwrap(),
        }
    }
}

impl mlua::UserData for UIInstance{
    fn add_methods<M: mlua::prelude::LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("run", |_,this, ()|{
            match this.instance.run(){
                Ok(_) => return Ok(()),
                Err(_) => Err("failed to run component".into_lua_err()),
            }
        });

        methods.add_method("get", |context : &mlua::Lua,this,property : mlua::LuaString|{
            let property_string = property.to_string_lossy();
            match this.instance.get_property(&property_string){
                Ok(property) => {
                    match slint_to_luau(context,&property){
                        Ok(value) => Ok(value),
                        Err(err) => Err(format!("Failed to get property '{}'\n{}",property_string,err).into_lua_err())
                    }
                },
                Err(err) => Err(format!("Failed to get property '{}'\n{}",property_string,err).into_lua_err()),
            }
        });

        methods.add_method_mut("set", |_,this,(property,value) : (mlua::LuaString,mlua::Value)|{
            let property_string = property.to_string_lossy();

            let converted = luau_to_slint(value)?;
            match this.instance.set_property(&property_string, converted){
                Ok(_) => Ok(()), 
                Err(err) => Err(format!("could not set property '{}'\n{}",property_string,err).into_lua_err()),
            }
        });

        methods.add_method_mut("connect", |context : &mlua::Lua,this,(callback_luauname,callbackfn) : (mlua::LuaString,mlua::Function)|{
            let callback_name = &callback_luauname.to_str()?;

            let key = context.create_registry_value(callbackfn)?;

            let context_clone = context.clone();

            this.instance.set_callback(callback_name, move |args|{
                let callback : mlua::Function = context_clone.registry_value(&key).unwrap();
                let luau_args = args.iter()
                                    .map(|value| slint_to_luau(&context_clone, value))
                                    .collect::<mlua::Result<Vec<_>>>();
                
                let callback_result = callback.call::<_>(mlua::MultiValue::from_vec(luau_args.unwrap())).expect("callback function errored");

                luau_to_slint(callback_result).unwrap()
            }).into_lua_err()
        });
    
        methods.add_method("getGlobal", |context : &mlua::Lua,this,(globalname,property) : (mlua::LuaString,mlua::LuaString)|{
            let property_string = property.to_string_lossy();
            let global_string = globalname.to_string_lossy();

            match this.instance.get_global_property(&global_string,&property_string){
                Ok(property) => {
                    match slint_to_luau(context,&property){
                        Ok(value) => Ok(value),
                        Err(err) => Err(format!("Failed to get global property '{}'\n{}",property_string,err).into_lua_err())
                    }
                },
                Err(err) => Err(format!("Failed to get global property '{}'\n{}",property_string,err).into_lua_err()),
            }
        });

        methods.add_method_mut("setGlobal", |_,this,(globalname,property,value) : (mlua::LuaString,mlua::LuaString,mlua::Value)|{
            let global_string = globalname.to_string_lossy();
            let property_string = property.to_string_lossy();

            let converted = luau_to_slint(value)?;
            match this.instance.set_global_property(&global_string,&property_string, converted){
                Ok(_) => Ok(()), 
                Err(err) => Err(format!("could not set global property '{}'\n{}",property_string,err).into_lua_err()),
            }
        });

    }
}