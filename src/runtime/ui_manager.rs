use slint_interpreter as slint;

pub struct UIManager {
    compiler: slint::Compiler,
}

impl UIManager {
    pub fn new() -> Self {
        Self {
            compiler: slint::Compiler::default(),
        }
    }
}
