pub trait DiscountStrategy {
    fn calculate_discount(&self, price: f64) -> f64;
}

pub struct NoDiscount;
pub struct PercentageDiscount(pub f64);
pub struct FixedAmountDiscount(pub f64);

impl DiscountStrategy for NoDiscount {
    fn calculate_discount(&self, _price: f64) -> f64 {
        0.0
    }
}

impl DiscountStrategy for PercentageDiscount {
    fn calculate_discount(&self, price: f64) -> f64 {
        price * (self.0 / 100.0)
    }
}

impl DiscountStrategy for FixedAmountDiscount {
    fn calculate_discount(&self, _price: f64) -> f64 {
        self.0
    }
}

pub struct Product {
    price: f64,
    discount_strategy: Box<dyn DiscountStrategy>,
}

impl Product {
    pub fn new(price: f64, discount_strategy: Box<dyn DiscountStrategy>) -> Self {
        Self {
            price,
            discount_strategy,
        }
    }

    pub fn price_with_discount(&self) -> f64 {
        self.price - self.discount_strategy.calculate_discount(self.price)
    }
}
