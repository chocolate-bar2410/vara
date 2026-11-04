pub struct AssertionEngine{
    assertions: Vec<AssertRecord>,
    failed: bool,
}

impl AssertionEngine {
    pub fn new() -> Self{
        Self {
            assertions: Vec::new(),
            failed: true,
        }
    }
}

pub struct AssertRecord {
    pub message: String,
    pub expected: String,
    pub actual: String,
}