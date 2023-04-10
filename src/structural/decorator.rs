pub trait Beverage {
    fn description(&self) -> String; 
    fn cost(&self) -> f64;
}

pub struct Coffee {
    description: String,
}

impl Coffee {
    pub fn new() -> Self {
        Coffee {
            description: "Coffee".to_string(),
        }
    }
}

impl Beverage for Coffee {
    fn description(&self) -> String {
        self.description.clone()
    }

    fn cost(&self) -> f64 {
        1.0
    }
}

pub trait BeverageDecorator: Beverage {
    fn new(beverage: Box<dyn Beverage>) -> Self;
}

pub struct Milk {
    pub beverage: Box<dyn Beverage>,
}

impl BeverageDecorator for Milk {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Milk { beverage }
    }
}

impl Beverage for Milk {
    fn description(&self) -> String {
        format!("{} with {}", self.beverage.description(), "Milk")
    }

    fn cost(&self) -> f64 {
        0.2 + self.beverage.cost()
    }
}

pub struct Sugar {
    pub beverage: Box<dyn Beverage>,
}

impl BeverageDecorator for Sugar {
    fn new(beverage: Box<dyn Beverage>) -> Self {
        Sugar { beverage }
    }
}

impl Beverage for Sugar {
    fn description(&self) -> String {
        format!("{} with {}", self.beverage.description(), "Sugar")
    }

    fn cost(&self) -> f64 {
        0.1 + self.beverage.cost()
    }
}


