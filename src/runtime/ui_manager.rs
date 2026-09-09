use std::path::PathBuf;
use std::collections::HashMap;

use slint_interpreter as slint;

pub struct UIManager {
    compiler: slint::Compiler,
    definitions: HashMap<String,slint_interpreter::ComponentDefinition>
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            compiler: slint::Compiler::default(),
            definitions: HashMap::new()
        }
    }

    pub fn consume_file(&mut self, filepath: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let compile_results = spin_on::spin_on(self.compiler.build_from_path(filepath));

        if compile_results.has_errors() {
            let mut output_message: String = format!("failed to compile file: {}\n", filepath.display()).to_string();

            for message in compile_results.diagnostics() {
                output_message.push_str(message.message());
            }

            return Err(output_message.into());
        }

        let names : Vec<&str> = compile_results.component_names().collect();
        
        for name in &names {
            if let Some(defintion) = compile_results.component(name) {
                self.definitions.insert(name.to_string(), defintion);
                println!(format!("loaded component {}",name));
            }
        }

        Ok(())
    }
}
