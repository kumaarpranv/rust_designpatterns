pub struct CarDirector;

pub struct Car {
    pub make: String,
    pub year: u32,
}

pub struct CarBuilder {
    make: Option<String>,
    year: Option<u32>,
}

impl CarBuilder {
    pub fn new() -> Self {
        CarBuilder {
            make: None,
            year: None,
        }
    }

    pub fn make(&mut self, make: &str) -> &mut Self {
        self.make = Some(make.to_string());
        self
    }

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.year = Some(year);
        self
    }

    pub fn build(&mut self) -> Result<Car, &'static str> {
        match (self.make.as_ref(), self.year) {
            (Some(make), Some(year)) => Ok(Car {
                make: make.clone(),
                year,
            }),
            _ => Err("Missing required fields"),
        }
    }
}

impl CarDirector {
    pub fn construct_sedan(builder: &mut CarBuilder) -> Result<Car, &'static str> {
        builder
            .make("Toyota")
            .year(2023)
            .build()
    }

    pub fn construct_sports_car(builder: &mut CarBuilder) -> Result<Car, &'static str> {
        builder
            .make("Porsche")
            .year(2023)
            .build()
    }
}
