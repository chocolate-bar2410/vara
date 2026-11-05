use std::collections::HashSet;


pub fn compare_tables(tab_a: &mlua::Table,tab_b: &mlua::Table) -> mlua::Result<bool>{
    let mut visited = HashSet::new();

    compare_table_recursive(tab_a, tab_b, &mut visited)
}

fn compare_table_recursive(tab_a: &mlua::Table,tab_b: &mlua::Table,visited: &mut HashSet<(*const std::ffi::c_void,*const std::ffi::c_void)>) -> mlua::Result<bool>{
    let pair = (tab_a.to_pointer(),tab_b.to_pointer());

    if !visited.insert(pair){
        return Ok(true);
    }

    for pair in tab_a.pairs::<mlua::Value,mlua::Value>() {
        let (key,value_a) = pair?;

        if !tab_b.contains_key(&key)? {return Ok(false)}
        let value_b = tab_b.get::<mlua::Value>(&key)?;

        if value_a.type_name() != value_b.type_name() {
            return Ok(false)
        }

         match (&value_a,&value_b) {
            (mlua::Value::Table(a), mlua::Value::Table(b)) => {
                if !compare_table_recursive(&a, &b, visited)? {
                    return Ok(false);
                }
            },

            _ => if !value_a.equals(&value_b)? { 
                return Ok(false);
            },
        };
    }


    for pair in tab_b.pairs::<mlua::Value,mlua::Value>() {
        let (key,_) = pair?;

        if !tab_a.contains_key(&key)? {
            return Ok(false);
        }
    }


    Ok(true)
}
