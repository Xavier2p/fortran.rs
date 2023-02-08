use super::{Var, Variable};

impl Var<String> for Variable<String> {
    fn new(name: String, value: String) -> Self {
        Variable { name, value }
    }

    fn get_value(&self) -> String {
        self.value.to_string()
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_value(&mut self, value: String) {
        self.value = value;
    }
}