use mlua::ExternalError;
use slint_interpreter::{self as slint, ComponentHandle};

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
                    match property{
                        slint::Value::Void => Ok(mlua::Value::Nil),
                        slint::Value::Bool(value) => Ok(mlua::Value::Boolean(value)),
                        slint::Value::String(value) => Ok(mlua::Value::String(
                            context.create_string(value.as_str())?
                        )),
                        slint::Value::Number(value) => Ok(mlua::Value::Number(value)),

                        _ => Err(format!("property {} has slint type that cannot be translated to luau",property_string).into_lua_err()),
                    }

                },
                Err(err) => Err(format!("Failed to get property {}\n{}",property_string,err).into_lua_err()),
            }
        });
    }
}