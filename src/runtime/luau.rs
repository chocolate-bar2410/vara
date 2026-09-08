use std::path::{PathBuf};

use mlua::{Lua, LuaOptions, StdLib};

use crate::stdlib;
pub struct Runtime{
    vm : Lua,
}

impl Runtime {
    pub fn new() -> Self {
        let lua = Lua::new_with(StdLib::ALL,LuaOptions::default()).unwrap();

        let mut runtime = Self{vm : lua};
        runtime.setup_globals().unwrap();

        runtime
    }

    fn setup_globals(&mut self) -> Result<(),Box<dyn std::error::Error>>{
        let globals = self.vm.globals();

        let io_lib = self.vm.create_table()?;

        io_lib.set("write",self.vm.create_function(stdlib::stdio::write)?)?;
        io_lib.set("read",self.vm.create_function(stdlib::stdio::read)?)?;
        io_lib.set("readline",self.vm.create_function(stdlib::stdio::readline)?)?;

        let fs_lib = self.vm.create_table()?;
        fs_lib.set("read",self.vm.create_function(stdlib::file::read_file_fn)?)?;
        fs_lib.set("write",self.vm.create_function(stdlib::file::write_file_fn)?)?;

        globals.set("fs",fs_lib)?;

        Ok(())
    }

    pub fn run(&mut self,file_path : &PathBuf) -> Result<(),Box<dyn std::error::Error>> {
        let file_contents = stdlib::file::read_file(file_path)?;
        self.vm.load(file_contents.as_str()).exec()?;
        Ok(())
    }
}