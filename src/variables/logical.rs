use super::{Var, Variable};

impl Var<bool> for Variable<bool> {
    fn new(name: String, value: bool) -> Self {
        Variable { name, value }
    }

    fn get_value(&self) -> bool {
        self.value
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_value(&mut self, value: bool) {
        self.value = value;
    }
}