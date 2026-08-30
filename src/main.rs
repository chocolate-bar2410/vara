use mlua::Lua;
use std::fs;
use std::io::{self, Read};

fn read_file(path : &str) -> io::Result<String> {

    let mut file = fs::File::open(path).expect(format!("Could not find file at path {}", path).as_str());
    let mut contents : String = String::new();

    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() -> Result<(),Box<dyn std::error::Error>> {
    let luau_vm = Lua::new();
    let file_contents = read_file("./demo_scripts/demo1.luau").expect("Failed to load file");

    let chunk = luau_vm.load(file_contents.as_str());

    chunk.exec()?;

    Ok(())
}
