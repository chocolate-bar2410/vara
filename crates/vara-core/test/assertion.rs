pub struct AssertionEngine{
    pub assertions: Vec<AssertRecord>,
    pub failed: bool,
}

impl AssertionEngine {
    pub fn new() -> Self{
        Self {
            assertions: Vec::new(),
            failed: false,
        }
    }

    pub fn record_assert(&mut self,record: AssertRecord){
        self.assertions.push(record);
        self.failed = true;
    }

    pub fn reset(&mut self){
        self.assertions.clear();
        self.failed = false;
    }
}

pub struct AssertRecord {
    pub message: String,
}