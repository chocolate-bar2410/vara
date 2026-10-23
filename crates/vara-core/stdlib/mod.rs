use crate::runtime::luau::Runtime;

pub mod serde;
pub mod stdio;
pub mod system;
pub mod task;
pub mod path;
pub mod terminal;

pub fn register_libaries(runtime: &Runtime) -> mlua::Result<()>{
    let io_lib = stdio::get_module(&runtime.vm)?;
    let serde_lib = serde::get_module(&runtime.vm)?;
    let system_lib = system::get_module(&runtime.vm,&runtime.args)?;
    let task_lib = task::get_module(&runtime.vm)?;
    let path_lib = path::get_module(&runtime.vm)?;
    let terminal_lib = terminal::get_module(&runtime.vm)?;
    
    runtime.vm.register_module("@vara/io", io_lib)?;
    runtime.vm.register_module("@vara/serde", serde_lib)?;
    runtime.vm.register_module("@vara/system", system_lib)?;
    runtime.vm.register_module("@vara/task", task_lib)?;
    runtime.vm.register_module("@vara/path", path_lib)?;
    runtime.vm.register_module("@vara/terminal", terminal_lib)?;

    Ok(())
}