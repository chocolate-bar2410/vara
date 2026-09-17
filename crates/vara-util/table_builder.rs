use mlua::IntoLua;

struct TableBuilder {
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
}
