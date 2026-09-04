use std::io::{self, Read, Write};

use std::path::{Path,PathBuf};

use mlua::{Lua,StdLib,LuaOptions};
use mlua::luau::{FsRequirer};

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

        io_lib.set("write",self.vm.create_function(|_, arg : mlua::LuaString | {

            let message = arg.as_bytes();

            io::stdout().write_all(&message).unwrap();

            Ok(())
        })?)?;

        io_lib.set("read",self.vm.create_function(|context,chunk_size: Option<usize>| {
            io::stdout().flush().map_err(mlua::Error::external)?;
            
            let mut stdin = io::stdin().lock();
            let size = chunk_size.unwrap_or(1024);
            let mut buffer = vec![0;size];

            let bytes_read = stdin.read(&mut buffer).map_err(mlua::Error::external)?;

            buffer.truncate(bytes_read);

            context.create_string(&buffer)
        })?)?;

        globals.set("io",io_lib)?;

        Ok(())
    }

    pub fn run(&mut self,file_path : &PathBuf) -> Result<(),Box<dyn std::error::Error>> {
        let file_contents = stdlib::file::read_file_with_pathbuf(file_path)?;
        self.vm.load(file_contents.as_str()).exec()?;
        Ok(())
    }
}