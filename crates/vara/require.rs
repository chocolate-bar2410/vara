use std::{fs, path::PathBuf};

pub struct Resolver {
    relative: PathBuf,
    absolute: PathBuf,
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            relative: PathBuf::new(),
            absolute: PathBuf::new(),
        }
    }

    fn navigate_to(
        &mut self,
        relative: PathBuf,
        absolute: PathBuf,
    ) -> Result<(), mlua::luau::NavigateError> {
        if self.relative == relative && self.absolute == absolute {
            return Ok(());
        }

        todo!()
    }

    fn nav_reset(&mut self) {
        self.relative = PathBuf::new();
        self.absolute = PathBuf::new();
    }
}

impl mlua::luau::Require for Resolver {
    fn is_require_allowed(&self, chunk_name: &str) -> bool {
        chunk_name.starts_with("@")
    }

    fn reset(&mut self, chunk_name: &str) -> Result<(), mlua::luau::NavigateError> {
        self.nav_reset();

        todo!()
    }

    fn jump_to_alias(&mut self, path: &str) -> Result<(), mlua::luau::NavigateError> {
        todo!()
    }

    fn to_parent(&mut self) -> Result<(), mlua::luau::NavigateError> {
        todo!()
    }

    fn to_child(&mut self, name: &str) -> Result<(), mlua::luau::NavigateError> {
        let relative = self.relative.join(name);
        let absolute = self.absolute.join(name);

        self.navigate_to(relative, absolute)
    }

    fn has_module(&self) -> bool {
        todo!()
    }

    fn cache_key(&self) -> String {
        todo!()
    }

    fn has_config(&self) -> bool {
        self.absolute.is_dir() && self.absolute.join(".luaurc").is_file()
    }

    fn config(&self) -> std::io::Result<Vec<u8>> {
        fs::read(self.absolute.join(".luaurc"))
    }

    fn loader(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        todo!()
    }
}
