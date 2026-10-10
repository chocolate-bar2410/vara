use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;

use vara_core::stdlib as vara_stdlib;

use crate::require;

pub struct Runtime {
    vm: Lua,
    args: Vec<String>
}

impl Runtime {
    pub fn new(args: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new_with(StdLib::ALL, LuaOptions::default()).unwrap();

        let mut runtime = Self { vm: lua,args: args };

        match runtime.setup_standard_library() {
            Ok(_) => Ok(runtime),
            Err(err) => Err(format!("failed to initialise standard library\n{err}").into()),
        }
    }

    fn setup_standard_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let io_lib = vara_stdlib::stdio::get_module(&self.vm)?;
        let fs_lib = vara_stdlib::files::get_module(&self.vm)?;
        let serde_lib = vara_stdlib::serde::get_module(&self.vm)?;
        let chrono_lib = vara_stdlib::chrono::get_module(&self.vm)?;
        let system_lib = vara_stdlib::system::get_module(&self.vm,&self.args)?;
        let media_lib = vara_stdlib::media::get_module(&self.vm)?;

        self.vm.register_module("@vara/io", io_lib)?;
        self.vm.register_module("@vara/fs", fs_lib)?;
        self.vm.register_module("@vara/serde", serde_lib)?;
        self.vm.register_module("@vara/chrono", chrono_lib)?;
        self.vm.register_module("@vara/system", system_lib)?;
        self.vm.register_module("@vara/media", media_lib)?;

        self.vm.globals().set(
            "require",
            self.vm
                .create_require_function(require::Resolver::new())
                .map(mlua::Value::Function)?,
        )?;

        Ok(())
    }


    pub fn run_bytecode(&self,bytecode : Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.vm.load(bytecode).exec()?;
        Ok(())
    }

    pub fn run(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = vara_stdlib::files::read_file(file_path)?;

        let module_path =
            vara_util::path::LuauModule::resolve(vara_util::path::LuauModule::strip(file_path))
                .map_err(|e| mlua::Error::external(format!("{e:?}")))?;

        self.vm
            .load(file_contents.as_str())
            .set_name(format!(
                "{}{}",
                vara_util::constants::FILE_CHUNK_PREFIX,
                module_path
            ))
            .exec()?;
        Ok(())
    }
}
