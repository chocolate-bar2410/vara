use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
struct LoaderState {
    loaded: HashMap<PathBuf, mlua::MultiValue>,
    loading: HashSet<PathBuf>,
}

impl LoaderState {
    pub fn new() -> Self {
        Self {
            loaded: HashMap::new(),
            loading: HashSet::new(),
        }
    }
}

pub struct RequireLoader {
    state: LoaderState,
}

impl RequireLoader {
    pub fn new() -> Self {
        Self {
            state: LoaderState::new(),
        }
    }

    pub fn load(&mut self, context: &mlua::Lua, relative: PathBuf, absolute: PathBuf) {
        let relative = relative.to_path_buf();
        let absolute = absolute.to_path_buf();
    }
}
