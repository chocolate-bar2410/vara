use std::collections::hash_map::HashMap;
use mlua::ExternalResult;

use crate::test::{unit};

pub struct TestRunner{
    tests: u32,
    passed: u32,
    failed: u32,

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

    pub fn evaluate(&mut self,name: String,test: mlua::Function) -> mlua::Result<()>{
        self.tests += 1;

        {
            let mut assert_engine = unit::get_assertion_engine()?;
            assert_engine.reset();
        }

        test.call::<()>(())?;

        let assert_engine = unit::get_assertion_engine()?;

        if !assert_engine.failed {
            self.passed += 1;

            self.message_map.insert(name.clone(), "passed.".to_string());
            self.result_map.insert(name, true);
        } else {
            let first_record = assert_engine.assertions.get(0).ok_or_else(|| unreachable!()).into_lua_err()?;

            self.failed += 1;

            self.message_map.insert(name.clone(), first_record.message.clone());
            self.result_map.insert(name, false);
        }

        Ok(())
    }

    pub fn get_result_output(&self) -> String {
        let mut result = String::new();

        for key in self.result_map.keys() {

            let passed = *self.result_map.get(key).unwrap_or(&false);
            let prefix = if passed {"PASS"} else {"FAIL"};

            let unknown_message = &String::from("???");            
            let message = self.message_map.get(key).unwrap_or(unknown_message);

            result.push_str(&format!("{} {} {}\n",prefix,key,message));
        }

        result
    }
}