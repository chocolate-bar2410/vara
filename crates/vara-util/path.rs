use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LuauFile {
    File(PathBuf),
    Directory(PathBuf),
}

impl LuauFile {
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

    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    pub const fn is_dir(&self) -> bool {
        matches!(self, Self::Directory(_))
    }

    pub fn as_file(&self) -> Option<&Path> {
        match self {
            Self::File(path) => Some(path),
            Self::Directory(_) => None,
        }
    }

    pub fn as_dir(&self) -> Option<&Path> {
        match self {
            Self::Directory(path) => Some(path),
            Self::File(_) => None,
        }
    }
}

impl AsRef<Path> for LuauFile {
    fn as_ref(&self) -> &Path {
        match self {
            Self::File(path) | Self::Directory(path) => path.as_ref(),
        }
    }
}

impl std::fmt::Display for LuauFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(path) | Self::Directory(path) => path.display().fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LuauModule {
    source: PathBuf,
    target: LuauFile,
}

impl LuauModule {
    pub fn strip(path: impl Into<PathBuf>) -> PathBuf {
        let mut path: PathBuf = path.into();

        if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| super::constants::FILE_EXTENSIONS.contains(&e))
        {
            path = path.with_extension("");
        }

        if path
            .file_name()
            .and_then(|e| e.to_str())
            .is_some_and(|f| f == super::constants::FILE_NAME_INIT)
        {
            path.pop();
        }

        path
    }

    pub fn resolve(module: impl Into<PathBuf>) -> Result<Self, mlua::luau::NavigateError> {
        let source = module.into();
        let target = LuauFile::resolve(&source)?;
        Ok(Self { source, target })
    }

    pub fn get_source(&self) -> &Path {
        &self.source
    }

    pub fn get_target(&self) -> &LuauFile {
        &self.target
    }
}

impl std::fmt::Display for LuauModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.get_source().display().fmt(f)
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
