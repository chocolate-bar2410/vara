use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(Debug, Clone)]
struct LoaderState {
    loaded: Rc<RefCell<HashMap<PathBuf, mlua::MultiValue>>>,
    loading: Rc<RefCell<HashSet<PathBuf>>>,
}

impl LoaderState {
    pub fn new() -> Self {
        Self {
            loaded: Rc::new(RefCell::new(HashMap::new())),
            loading: Rc::new(RefCell::new(HashSet::new())),
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

    pub fn load(
        &mut self,
        context: &mlua::Lua,
        relative: &Path,
        absolute: &Path,
    ) -> mlua::Result<mlua::Function> {
        let relative = relative.to_path_buf();
        let absolute = absolute.to_path_buf();

        let state = self.state.clone();

        context.create_function(move |context, ()| {
            if let Some(result) = state.loaded.borrow().get(&absolute) {
                return Ok(result.clone());
            }

            if state.loading.borrow().contains(&absolute) {
                return Err(mlua::Error::RuntimeError(format!(
                    "circular require detected {}",
                    absolute.display()
                )));
            }

            state.loading.borrow_mut().insert(absolute.clone());

            let chunk_bytes =
                fs::read(&absolute).map_err(|e| mlua::Error::RuntimeError(e.to_string()))?;

            let chunk_name = format!("chunk: {}", relative.display());
            let chunk = context.load(&chunk_bytes).set_name(chunk_name);

            let result = chunk.call::<mlua::MultiValue>(())?;

            state
                .loaded
                .borrow_mut()
                .insert(absolute.clone(), result.clone());
            state.loading.borrow_mut().remove(&absolute);

            Ok(result)
        })
    }
}
