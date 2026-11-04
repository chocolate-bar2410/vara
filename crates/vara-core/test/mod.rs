use crate::runtime::luau::Runtime;

mod test_runner;
mod unit;
mod assertion;

pub use test_runner::TestRunner;

pub fn register_libraries(runtime: &Runtime) -> mlua::Result<()>{
    let unit_lib = unit::get_module(&runtime.vm)?;
    
    runtime.vm.register_module("@vara/test/unit", unit_lib)?;

    Ok(())
}