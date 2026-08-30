use std::path::{self, Path};

use mlua::{Lua,StdLib,LuaOptions};
use mlua::luau::{FsRequirer};

use crate::util;

pub struct Runtime{
    vm : Lua,
}

impl Runtime {
    pub fn new() -> Self {
        let lua = Lua::new_with(StdLib::ALL,LuaOptions::default()).unwrap();

        Self{vm : lua}
    }

    pub fn run(&mut self,file_path : &str) -> Result<(),Box<dyn std::error::Error>> {
        let file_contents = util::file::read_file(file_path)?;
        self.vm.load(file_contents.as_str()).exec()?;
        Ok(())
    }
}