use std::collections::hash_map::HashMap;
use mlua::ExternalResult;

use crate::test::{unit};

pub struct TestRunner{
    pub tests: u32,
    pub passed: u32,
    pub failed: u32,

    message_map: HashMap<String,String>,
    result_map: HashMap<String,bool>
}

impl TestRunner{
    pub fn new() -> Self{
        Self { 
            tests: 0,
            passed: 0,
            failed: 0,
            message_map: HashMap::new(),
            result_map: HashMap::new()
        }
    }

    pub fn evaluate(&mut self,name: String,test: mlua::Function) -> mlua::Result<bool>{
        self.tests += 1;

        {
            let mut assert_engine = unit::get_assertion_engine()?;
            assert_engine.reset();
        }

        test.call::<()>(())?;

        let assert_engine = unit::get_assertion_engine()?;

        if !assert_engine.failed {
            self.passed += 1;

            self.message_map.insert(name.clone(), "".to_string());
            self.result_map.insert(name, true);
        } else {
            let first_record = assert_engine.assertions.get(0).ok_or_else(|| unreachable!()).into_lua_err()?;

            self.failed += 1;

            self.message_map.insert(name.clone(), first_record.message.clone());
            self.result_map.insert(name, false);
        }

        Ok(assert_engine.failed)
    }

    pub fn get_result_output(&self,suite_name: &str) -> String {
        let mut result = self.result_map
            .iter()
            .map(|(key,passed)| {
                let prefix = match passed {
                    true => "\x1b[1;32mPASS\x1b[0m",
                    false => "\x1b[1;31mFAIL\x1b[0m"
                };

                let unknown_message = &String::from("???");            
                let message = self.message_map
                .get(key)
                    .map(String::as_str)
                    .unwrap_or(unknown_message);

                if *passed {
                    return format!("{} {} > {}",prefix,suite_name,key);
                }

                format!("{} {} > {}: {}",prefix,suite_name,key,message)
            })
            .collect::<Vec<String>>()
            .join("\n");

        result.push_str(&format!("\n\n{}{} pass\x1b[0m\n",if self.passed > 0 {"\x1b[32m"} else {"\x1b[31m"},self.passed));
        result.push_str(&format!("{}{} fail\x1b[0m\n",if self.failed > 0 {"\x1b[31m"} else {"\x1b[2m"},self.failed));
   
        result
    }
}