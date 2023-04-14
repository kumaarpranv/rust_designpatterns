pub trait Command {
    fn execute(&self) -> f64;
}

#[derive(Clone)]
pub struct Calculator {
    pub value: f64,
}

pub struct AddCommand {
    calculator: Calculator,
    value: f64,
}

impl Command for AddCommand {
    fn execute(&self) -> f64 {
        self.calculator.value + self.value
    }
}

impl AddCommand {
    pub fn new(calculator: Calculator, value: f64) -> Self {
        AddCommand { calculator, value }
    }
}

pub struct SubtractCommand {
    calculator: Calculator,
    value: f64,
}

impl Command for SubtractCommand {
    fn execute(&self) -> f64 {
        self.calculator.value - self.value
    }
}

impl SubtractCommand {
    pub fn new(calculator: Calculator, value: f64) -> Self {
        SubtractCommand { calculator, value }
    }
}


pub struct MultiplyCommand {
    calculator: Calculator,
    value: f64,
}

impl MultiplyCommand {
    pub fn new(calculator: Calculator, value: f64) -> Self {
        MultiplyCommand { calculator, value }
    }
}


impl Command for MultiplyCommand {
    fn execute(&self) -> f64 {
        self.calculator.value * self.value
    }
}

pub struct DivideCommand {
    calculator: Calculator,
    value: f64,
}

impl DivideCommand {
    pub fn new(calculator: Calculator, value: f64) -> Self {
        DivideCommand { calculator, value }
    }
}

impl Command for DivideCommand {
    fn execute(&self) -> f64 {
        self.calculator.value / self.value
    }
}
