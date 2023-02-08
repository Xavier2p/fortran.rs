use super::{Var, Variable};

impl Var<f64> for Variable<f64> {
    fn new(name: String, value: f64) -> Self {
        Variable { name, value }
    }

    fn get_value(&self) -> f64 {
        self.value
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn set_value(&mut self, value: f64) {
        self.value = value;
    }
}
