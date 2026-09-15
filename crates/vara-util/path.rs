use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuauPath {
    File(PathBuf),
    Directory(PathBuf),
}

impl LuauPath {
    fn resolve(module: impl AsRef<Path>) -> Result<Self, mlua::luau::NavigateError> {
        let module = module.as_ref();

        if module
            .file_name()
            .is_some_and(|n| n == OsStr::new(super::constants::FILE_NAME_INIT))
        {
            return Err(mlua::luau::NavigateError::Ambiguous);
        }

        let mut found = None;

        for ext in super::constants::FILE_EXTENSIONS {
            let candidate = append_extension(module, ext);
            if candidate.is_file() && found.replace(candidate).is_some() {
                return Err(mlua::luau::NavigateError::Ambiguous);
            }
        }

        if module.is_dir() {
            let init = Path::new(super::constants::FILE_NAME_INIT);
            for ext in super::constants::FILE_EXTENSIONS {
                let candidate = append_extension(init, ext);
                if candidate.is_file() && found.replace(candidate).is_some() {
                    return Err(mlua::luau::NavigateError::Ambiguous);
                }
            }

            if found.is_none() {
                return Ok(Self::Directory(module.to_path_buf()));
            }
        }

        found
            .map(Self::File)
            .ok_or(mlua::luau::NavigateError::NotFound)
    }
}

fn append_extension(path: impl AsRef<Path>, ext: impl AsRef<OsStr>) -> PathBuf {
    let path = path.as_ref();
    match path.extension() {
        None => path.with_extension(ext),
        Some(curr_ext) => {
            let mut new_ext = curr_ext.to_os_string();
            new_ext.push(".");
            new_ext.push(ext);
            path.with_extension(new_ext)
        }
    }
}
