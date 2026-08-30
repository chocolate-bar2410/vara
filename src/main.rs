mod luau;
mod util;

fn main() -> Result<(),Box<dyn std::error::Error>> {

    let mut current = luau::Runtime::new();

    current.run("./demo_scripts/demo1.luau")?;

    Ok(())
}
