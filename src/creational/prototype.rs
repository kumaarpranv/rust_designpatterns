pub trait Shape: Clone {
    fn area(&self) -> f64;
}

#[derive(Clone)]
pub struct Circle {
    pub radius: f64,
}

#[derive(Clone)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

}
