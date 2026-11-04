use std::{collections::HashSet, ops::Add};

use mlua::ObjectLike;

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
            mlua::Value::Buffer(buffer) => result += print_buffer(buffer)?.as_str(),
            _ => result += &param.to_string()?,
        }
    }

    Ok(result)
}

fn print_table(table: &mlua::Table) -> mlua::Result<String>{

    if let Some(meta) = table.metatable() {
        if meta.contains_key("__tostring")? {
            return table.to_string();
        }
    }

    let mut visited = HashSet::new();

    print_table_recursive(table, &mut visited,1)
}

fn print_table_recursive(table: &mlua::Table,visited: &mut HashSet<*const std::ffi::c_void>,depth: usize) -> mlua::Result<String>{
    let mut result = String::new();

    let indent_str = "  ";

    result += &"{\n";

    for pair in table.pairs::<mlua::Value,mlua::Value>() {

        let (key,value) = pair?;

        result.push_str(&indent_str.repeat(depth));

        let formatted = match key {
            mlua::Value::String(_) => format!("\x1b[32m\"{}\"\x1b[0m",key.to_string()?),
            mlua::Value::Number(_) | mlua::Value::Integer(_) => format!("\x1b[36m{}\x1b[0m",key.to_string()?),
            _ => key.to_string()?
        };

        result.push_str(&format!("[{}]: ",formatted));
        
        match value {
            mlua::Value::Table(tab) => {
                let ptr = tab.to_pointer();
                if visited.contains(&ptr) {
                    result.push_str("**CYCLICAL REFERENCE**,\n")
                } else {
                    visited.insert(ptr);
                    result.push_str(&format!("{},\n",print_table_recursive(&tab,visited,depth + 1)?));
                    visited.remove(&ptr);
                }
            }

            _ => result.push_str(&format!("{},\n",value.to_string()?)),
        }
    }

    result += &indent_str.repeat(depth - 1).add("}");

    Ok(result)
}

fn print_buffer(buffer: &mlua::Buffer) -> mlua::Result<String>{
    let mut result = String::new();
    const MAX_WIDTH: usize = 16;
    const BYTE_SECT_WIDTH: usize = 50;
    const ENCODED_SECT_WIDTH: usize = 16;

    let bytedump = buffer.to_vec();

    let mut row = 0;
    let mut bytes = String::new();
    let mut encoded = String::new();

    for (index,byte) in bytedump.iter().enumerate() {
        if index > 0 && index % MAX_WIDTH == 0 {

            result.push_str(&format!("{:08x?}  {}  | {} |\n",row,&bytes,&encoded));

            bytes.clear();
            encoded.clear();

            row += MAX_WIDTH;
        } else if index > 0 && index % MAX_WIDTH.div_ceil(2) == 0 {
            bytes.push_str("  ");
        }

        bytes.push_str(&format!("{:02x?} ",byte));

        if byte.is_ascii_alphanumeric() {
            encoded.push(*byte as char);
        } else {
            encoded.push('.')
        };
    }

    let padding_a = " ".repeat(BYTE_SECT_WIDTH - bytes.len());
    let padding_b = " ".repeat(ENCODED_SECT_WIDTH - encoded.len());

    result.push_str(&format!("{:08x?}  {}{}  | {}{} |",row,&bytes,&padding_a,&encoded,&padding_b));

    bytes.clear();
    encoded.clear();

    Ok(result)
} 