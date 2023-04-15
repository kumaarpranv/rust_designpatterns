use std::f64::consts::PI;

pub trait ShapeVisitor {
    fn visit_circle(&mut self, circle: &Circle);
    fn visit_rectangle(&mut self, rectangle: &Rectangle);
    fn visit_triangle(&mut self, triangle: &Triangle);
}

pub trait Shape {
    fn accept(&self, visitor: &mut dyn ShapeVisitor);
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Shape for Circle {
    fn accept(&self, visitor: &mut dyn ShapeVisitor) {
        visitor.visit_circle(self);
    }
}

impl Shape for Rectangle {
    fn accept(&self, visitor: &mut dyn ShapeVisitor) {
        visitor.visit_rectangle(self);
    }
}

impl Shape for Triangle {
    fn accept(&self, visitor: &mut dyn ShapeVisitor) {
        visitor.visit_triangle(self);
    }
}

pub struct AreaCalculator {
    pub area: f64,
}

impl ShapeVisitor for AreaCalculator {
    fn visit_circle(&mut self, circle: &Circle) {
        self.area = PI * circle.radius * circle.radius;
    }

    fn visit_rectangle(&mut self, rectangle: &Rectangle) {
        self.area = rectangle.width * rectangle.height;
    }

    fn visit_triangle(&mut self, triangle: &Triangle) {
        self.area = 0.5 * triangle.base * triangle.height;
    }
}
