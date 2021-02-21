pub type Value = f32;

pub struct ValuesArray {
    values: Vec<Value>
}

impl ValuesArray {
    pub fn new() -> ValuesArray {
        ValuesArray {
            values: Vec::new(),
        }
    }

    pub fn write(&mut self, value: Value) {
        self.values.push(value);
    }
}