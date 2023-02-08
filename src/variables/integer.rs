use super::{Var, Variable};


impl Var<i32> for Variable<i32> {
    fn new(name: String, value: i32) -> Self {
        Variable { name, value }
    }

    fn get_value(&self) -> i32 {
        self.value
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_value(&mut self, value: i32) {
        self.value = value;
    }
}