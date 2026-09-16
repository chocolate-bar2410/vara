use std::{
    fs,
    path::{Path, PathBuf},
};
use vara_util::path::LuauModule;
use vara_util::path::clean_path_and_make_absolute;
use vara_util::path::relative_path_normalize;
use vara_util::path::relative_path_parent;

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
        let relative = relative_path_normalize(Path::new(path));
        let absolute = clean_path_and_make_absolute(&relative);

        self.navigate_to(relative, absolute)
    }

    fn to_parent(&mut self) -> Result<(), mlua::luau::NavigateError> {
        let mut relative = self.relative.clone();
        let mut absolute = self.absolute.clone();

        if absolute.pop() {
            relative_path_parent(&mut relative);
            return self.navigate_to(relative, absolute);
        }

        Err(mlua::luau::NavigateError::NotFound)
    }

    fn to_child(&mut self, name: &str) -> Result<(), mlua::luau::NavigateError> {
        let relative = self.relative.join(name);
        let absolute = self.absolute.join(name);

        self.navigate_to(relative, absolute)
    }

    fn has_module(&self) -> bool {
        let resolved = self.resolved.as_ref();

        resolved.is_some_and(|p| p.get_target().is_file())
    }

    fn cache_key(&self) -> String {
        let resolved = self.resolved.as_ref();
        resolved.expect("called has_module first").to_string()
    }

    fn has_config(&self) -> bool {
        self.absolute.is_dir()
            && self
                .absolute
                .join(vara_util::constants::FILE_NAME_CONFIG)
                .is_file()
    }

    fn config(&self) -> std::io::Result<Vec<u8>> {
        fs::read(self.absolute.join(vara_util::constants::FILE_NAME_CONFIG))
    }

    fn loader(&self, context: &mlua::Lua) -> mlua::Result<mlua::Function> {
        let resolved = self
            .resolved
            .as_ref()
            .expect("called has_module first")
            .get_target()
            .as_file()
            .expect("tried to require a directory");

        todo!()
    }
}
