use std::collections::hash_map::HashMap;

use mlua::ExternalError;

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

    pub fn evaluate(&mut self,name: String,test: mlua::Function){
        self.tests += 1;

        let result = test.call::<()>(());

        match result {
            Ok(_) => {
                self.passed += 1;

                self.message_map.insert(name.clone(), "passed.".to_string());
                self.result_map.insert(name, true);
            },
            Err(err) => {

                let message = match err{
                    mlua::Error::RuntimeError(msg) => msg.lines().next().unwrap_or(&msg).to_string(),
                    mlua::Error::CallbackError { cause, .. } => cause.to_string(),
                    _ => err.to_string()
                };

                self.failed += 1;

                self.message_map.insert(name.clone(), message);
                self.result_map.insert(name, false);
            }
        }
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