use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use vara_core::stdlib as vara_stdlib;
use vara_core::ui::ui_manager as vara_UI;

use vara_UI::UIManager;

pub struct Runtime {
    vm: Lua,
    ui: Arc<Mutex<UIManager>>,
}

impl Runtime {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();

        let mut runtime = Self {
            vm: lua,
            ui: Arc::new(Mutex::new(UIManager::new())),
        };

        match runtime.setup_standard_library() {
            Ok(_) => Ok(runtime),
            Err(err) => Err(format!("failed to initialise standard library\n{err}").into()),
        }
    }

    fn setup_standard_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let io_lib = self.vm.create_table()?;

        io_lib.set("write", self.vm.create_function(vara_stdlib::stdio::write)?)?;
        io_lib.set("read", self.vm.create_function(vara_stdlib::stdio::read)?)?;
        io_lib.set(
            "readline",
            self.vm.create_function(vara_stdlib::stdio::readline)?,
        )?;

        let fs_lib = self.vm.create_table()?;
        fs_lib.set(
            "read",
            self.vm.create_function(vara_stdlib::file::read_file_fn)?,
        )?;
        fs_lib.set(
            "write",
            self.vm.create_function(vara_stdlib::file::write_file_fn)?,
        )?;

        let gui_lib = self.vm.create_table()?;

        let ui_register_arc = Arc::clone(&self.ui);
        gui_lib.set(
            "registerFile",
            self.vm
                .create_function(move |_: &Lua, luau_path: mlua::LuaString| {
                    let filepath = PathBuf::from(luau_path.to_string_lossy());

                    let mut ui = ui_register_arc
                        .lock()
                        .map_err(|_| mlua::Error::external("UI manager mutex poisioned"))?;
                    ui.consume_file(&filepath).map_err(mlua::Error::external)
                })?,
        )?;

        let ui_create_arc = Arc::clone(&self.ui);
        gui_lib.set(
            "createInstance",
            self.vm
                .create_function(move |_: &Lua, definition_luaustr: mlua::LuaString| {
                    let definition = definition_luaustr.to_string_lossy();

                    let ui = ui_create_arc
                        .lock()
                        .map_err(|_| mlua::Error::external("UI manager mutex poisioned"))?;
                    ui.create_instance(definition)
                        .map_err(mlua::Error::external)
                })?,
        )?;

        self.vm.register_module("@vara/io", io_lib)?;
        self.vm.register_module("@vara/fs", fs_lib)?;
        self.vm.register_module("@vara/gui", gui_lib)?;

        Ok(())
    }

    pub fn run(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = vara_stdlib::file::read_file(file_path)?;
        self.vm.load(file_contents.as_str()).exec()?;
        Ok(())
    }
}
