pub struct CarDirector;

pub struct Car {
    pub make: String,
    model: String,
    year: u32,
    price: Option<f64>,
}

pub struct CarBuilder {
    make: Option<String>,
    model: Option<String>,
    year: Option<u32>,
    price: Option<f64>,
}

impl CarBuilder {
    pub fn new() -> Self {
        CarBuilder {
            make: None,
            model: None,
            year: None,
            price: None,
        }
    }

    pub fn make(&mut self, make: &str) -> &mut Self {
        self.make = Some(make.to_string());
        self
    }

    pub fn model(&mut self, model: &str) -> &mut Self {
        self.model = Some(model.to_string());
        self
    }

    pub fn year(&mut self, year: u32) -> &mut Self {
        self.year = Some(year);
        self
    }

    pub fn price(&mut self, price: f64) -> &mut Self {
        self.price = Some(price);
        self
    }

    pub fn build(&mut self) -> Result<Car, &'static str> {
        match (self.make.as_ref(), self.model.as_ref(), self.year) {
            (Some(make), Some(model), Some(year)) => Ok(Car {
                make: make.clone(),
                model: model.clone(),
                year,
                price: self.price,
            }),
            _ => Err("Missing required fields"),
        }
    }
}

impl CarDirector {
    pub fn construct_sedan(builder: &mut CarBuilder) -> Result<Car, &'static str> {
        builder
            .make("Toyota")
            .model("Corolla")
            .year(2023)
            .price(30000.0)
            .build()
    }

    pub fn construct_sports_car(builder: &mut CarBuilder) -> Result<Car, &'static str> {
        builder
            .make("Porsche")
            .model("911")
            .year(2023)
            .price(100000.0)
            .build()
    }
}
