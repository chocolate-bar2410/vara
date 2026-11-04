use std::{collections::HashSet, ops::Add};

pub fn print(_context: &mlua::Lua, params : mlua::MultiValue) -> mlua::Result<()>{
    println!("{}",to_print_string(_context,params)?);
    Ok(())
}

pub fn warn(_context: &mlua::Lua, params : mlua::MultiValue) -> mlua::Result<()>{
    println!("\x1b[33m{}\x1b[0m",to_print_string(_context,params)?);
    Ok(())
}

fn to_print_string(_context: &mlua::Lua, params : mlua::MultiValue) -> mlua::Result<String>{
    let mut result = String::new();

    let mut first = true;

    for param in params.iter() {
        if first {first = false} else {result += " "}

        match param {
            mlua::Value::Table(tab) => result += print_table(tab)?.as_str(),
            _ => result += &param.to_string()?,
        }
    }

    Ok(result)
}

fn print_table(table: &mlua::Table) -> mlua::Result<String>{
    let mut visited = HashSet::new();

    print_table_recursive(table, &mut visited,1)
}

fn print_table_recursive(table: &mlua::Table,visited: &mut HashSet<*const std::ffi::c_void>,depth: usize) -> mlua::Result<String>{
    let mut result = String::new();

    let indent_str = "  ";

    result += &indent_str.repeat(depth - 1).add("{\n");

    for pair in table.pairs::<mlua::Value,mlua::Value>() {

        let (key,value) = pair?;

        result.push_str(&indent_str.repeat(depth));

        match key {
            _ => {                
                result.push_str(&format!("[{}]: ",key.to_string()?))
            },
        }

        match value {
            mlua::Value::Table(tab) => {
                let ptr = tab.to_pointer();
                if visited.contains(&ptr) {
                    result.push_str("**CYCLICAL REFERENCE**")
                } else {
                    visited.insert(ptr);
                    result.push_str(&format!("{},\n",print_table_recursive(&tab,visited,depth + 1)?));
                }
            }

            _ => result.push_str(&format!("{},\n",value.to_string()?)),
        }
    }

    result += &indent_str.repeat(depth - 1).add("}");

    Ok(result)
}

