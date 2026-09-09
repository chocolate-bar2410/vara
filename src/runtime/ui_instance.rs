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
    }
}