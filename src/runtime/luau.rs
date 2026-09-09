use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;
use std::sync::{Arc,Mutex};

use crate::{runtime::ui_manager::UIManager, stdlib};
pub struct Runtime {
    vm: Lua,
    ui: Arc<Mutex<UIManager>>,
}

impl Runtime {
    pub fn new() -> Self {
        let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();

        let mut runtime = Self {
            vm: lua,
            ui: Arc::new(Mutex::new(UIManager::new())),
        };
        runtime.setup_globals().unwrap();

        runtime
    }

    fn setup_globals(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let globals = self.vm.globals();

        let io_lib = self.vm.create_table()?;

        io_lib.set("write", self.vm.create_function(stdlib::stdio::write)?)?;
        io_lib.set("read", self.vm.create_function(stdlib::stdio::read)?)?;
        io_lib.set(
            "readline",
            self.vm.create_function(stdlib::stdio::readline)?,
        )?;

        let fs_lib = self.vm.create_table()?;
        fs_lib.set("read", self.vm.create_function(stdlib::file::read_file_fn)?)?;
        fs_lib.set(
            "write",
            self.vm.create_function(stdlib::file::write_file_fn)?,
        )?;

        globals.set("fs", fs_lib)?;

        let gui_lib = self.vm.create_table()?;

        let ui_register_arc = Arc::clone(&self.ui);
        gui_lib.set("registerFile",self.vm.create_function(move |_ : &Lua,luau_path : mlua::LuaString|{
            let filepath = PathBuf::from(luau_path.to_string_lossy());

            let mut ui = ui_register_arc.lock().map_err(|_| mlua::Error::external("UI manager mutex poisioned"))?;
            ui.consume_file(&filepath).map_err(mlua::Error::external)
        })?)?;

        let ui_create_arc = Arc::clone(&self.ui);
        gui_lib.set("createInstance",self.vm.create_function(move |_ : &Lua,definition_luaustr : mlua::LuaString|{
            let definition = definition_luaustr.to_string_lossy();

            let ui = ui_create_arc.lock().map_err(|_| mlua::Error::external("UI manager mutex poisioned"))?;
            ui.create_instance(definition).map_err(mlua::Error::external)
        })?)?;


        globals.set("gui", gui_lib)?;

        Ok(())
    }

    pub fn run(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = stdlib::file::read_file(file_path)?;
        self.vm.load(file_contents.as_str()).exec()?;
        Ok(())
    }
}
