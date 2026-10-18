use mlua::{Lua, LuaOptions, StdLib};
use std::path::PathBuf;

use crate::stdlib as vara_stdlib;
use crate::runtime::require;

// omg lune thx so much :heart_eyes:
use mlua_luau_scheduler::Scheduler;
use mlua_luau_scheduler::Functions;

const VARA_VERSION: &str = env!("CARGO_PKG_VERSION");
const VARA_URL: &str = "https://github.com/chocolate_bar2410/vara";

pub struct Runtime {
    pub vm: Lua,
    pub args: Vec<String>,
    scheduler: Scheduler,
}

impl Runtime {
    pub fn new(args: Vec<String>) -> Result<Self, Box<dyn std::error::Error>> {
        let lua = Lua::new_with(StdLib::ALL, LuaOptions::default())?;

        let scheduler = Scheduler::new(lua.clone());

        let async_fns = Functions::new(lua.clone())?;

        let coroutine_lib = lua.globals().get::<mlua::Table>("coroutine")?;

        // so these functions work with the task scheduler
        coroutine_lib.set("resume",async_fns.resume.clone())?;
        coroutine_lib.set("wrap",async_fns.wrap.clone())?;

        let mut runtime = Self { 
            vm: lua,
            args,
            scheduler,
        };

        match runtime.setup_standard_library() {
            Ok(_) => Ok(runtime),
            Err(err) => Err(format!("failed to initialise standard library\n{err}").into()),
        }
    }

    fn setup_standard_library(&mut self) -> Result<(), Box<dyn std::error::Error>> {

        vara_stdlib::register_libaries(self)?;

        let fs_lib = vara_files::get_module(&self.vm)?;
        let chrono_lib = vara_chrono::get_module(&self.vm)?;
        let media_lib = vara_media::get_module(&self.vm)?;
        let net_lib = vara_net::get_module(&self.vm)?;
        let ui_lib = vara_ui::get_module(&self.vm)?;

        self.vm.register_module("@vara/fs", fs_lib)?;
        self.vm.register_module("@vara/chrono", chrono_lib)?;
        self.vm.register_module("@vara/media", media_lib)?;
        self.vm.register_module("@vara/net", net_lib)?;
        self.vm.register_module("@vara/ui", ui_lib)?;

        self.vm.globals().set(
            "require",
            self.vm
                .create_require_function(require::Resolver::new())
                .map(mlua::Value::Function)?,
        )?;

        let runtime_global = self.vm.create_table()?;
        let version_tab = self.vm.create_table()?;

        version_tab.set("display", VARA_VERSION)?;
        
        runtime_global.set("name", "vara")?;
        runtime_global.set("version", version_tab)?;
        runtime_global.set("url", VARA_URL)?;

        self.vm.globals().set("_RUNTIME", runtime_global)?;

        Ok(())
    }


    pub async fn run_bytecode(&self,bytecode : Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        self.vm.load(bytecode).exec_async().await?;

        Ok(())
    }

    pub async fn run(&mut self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let file_contents = vara_files::read_file(file_path)?;

        let module_path =
            vara_util::path::LuauModule::resolve(vara_util::path::LuauModule::strip(file_path))
                .map_err(|e| mlua::Error::external(format!("{e:?}")))?;

        let main = self.vm
            .load(file_contents.as_str())
            .set_name(format!(
                "{}{}",
                vara_util::constants::FILE_CHUNK_PREFIX,
                module_path
            ));

        self.scheduler.push_thread_back(main, ())?;

        smol::block_on(self.scheduler.run());

        Ok(())
    }
}
