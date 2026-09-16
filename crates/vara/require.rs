use std::{
    fs,
    path::{Path, PathBuf},
};
use vara_util::path::LuauModule;
use vara_util::path::clean_path_and_make_absolute;
use vara_util::path::relative_path_normalize;

pub struct Resolver {
    relative: PathBuf,
    absolute: PathBuf,
    resolved: Option<LuauModule>,
}

impl Resolver {
    pub(crate) fn new() -> Self {
        Self {
            relative: PathBuf::new(),
            absolute: PathBuf::new(),
            resolved: None,
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

        let resolved = LuauModule::resolve(&absolute)?;

        self.absolute = absolute;
        self.relative = relative;
        self.resolved = Some(resolved);

        Ok(())
    }

    fn nav_reset(&mut self) {
        self.relative = PathBuf::new();
        self.absolute = PathBuf::new();
        self.resolved = None;
    }
}

impl mlua::luau::Require for Resolver {
    fn is_require_allowed(&self, chunk_name: &str) -> bool {
        chunk_name.starts_with(vara_util::constants::FILE_CHUNK_PREFIX)
    }

    fn reset(&mut self, chunk_name: &str) -> Result<(), mlua::luau::NavigateError> {
        self.nav_reset();

        if let Some(path) = chunk_name.strip_prefix(vara_util::constants::FILE_CHUNK_PREFIX) {
            let relative = relative_path_normalize(Path::new(path));
            let absolute = clean_path_and_make_absolute(&relative);

            self.navigate_to(relative, absolute)
        } else {
            Err(mlua::luau::NavigateError::Other(mlua::Error::runtime(
                "cannot reset require state from non-file chunk",
            )))
        }
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
