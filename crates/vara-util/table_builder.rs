use std::future::Future;

pub struct TableBuilder {
    context: mlua::Lua,
    result: mlua::Table,
}

impl TableBuilder {
    pub fn new(context: mlua::Lua) -> mlua::Result<Self> {
        let tab = context.create_table()?;
        Ok(Self {
            context,
            result: tab,
        })
    }

    pub fn add_value<K, V>(self, key: K, value: V) -> mlua::Result<Self>
    where
        K: mlua::IntoLua,
        V: mlua::IntoLua,
    {
        self.result.set(key, value)?;
        Ok(self)
    }

    pub fn add_values<K, V>(self, keyvalues: Vec<(K, V)>) -> mlua::Result<Self>
    where
        K: mlua::IntoLua,
        V: mlua::IntoLua,
    {
        for (key, value) in keyvalues {
            self.result.set(key, value)?;
        }
        Ok(self)
    }

    pub fn add_function<A, R, K, F>(self, key: K, callback: F) -> mlua::Result<Self>
    where
        A: mlua::FromLuaMulti,
        R: mlua::IntoLuaMulti,
        K: mlua::IntoLua,
        F: Fn(&mlua::Lua, A) -> mlua::Result<R> + 'static + mlua::MaybeSend,
    {
        let luau_func = self.context.create_function(callback)?;
        self.result.set(key, mlua::Value::Function(luau_func))?;
        Ok(self)
    }

    pub fn add_async_function<A, R, K, F,Fut>(self, key: K, callback: F) -> mlua::Result<Self>
    where
        A: mlua::FromLuaMulti,
        R: mlua::IntoLuaMulti,
        K: mlua::IntoLua,
        F: Fn(mlua::Lua, A) -> Fut + 'static + mlua::MaybeSend,
        Fut: Future<Output = mlua::Result<R>> + 'static + mlua::MaybeSend
    {
        let luau_func = self.context.create_async_function(callback)?;

        self.result.set(key, mlua::Value::Function(luau_func))?;
        Ok(self)
    }

    pub fn build_readonly(self) -> mlua::Result<mlua::Table> {
        self.result.set_readonly(true);
        Ok(self.result)
    }

    pub fn build(self) -> mlua::Result<mlua::Table> {
        Ok(self.result)
    }
}
