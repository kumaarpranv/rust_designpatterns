use crate::behavioural::visitor::{AreaCalculator,Shape, Circle, Rectangle, Triangle};

use std::f64::consts::PI;
#[test]
fn test_circle_area() {
    let circle = Circle { radius: 3.0 };
    let mut area_calculator = AreaCalculator { area: 0.0 };

    circle.accept(&mut area_calculator);

    assert_eq!(area_calculator.area, 3.0 * 3.0 * PI);
}

#[test]
fn test_rectangle_area() {
    let rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    let mut area_calculator = AreaCalculator { area: 0.0 };

    rectangle.accept(&mut area_calculator);

    assert_eq!(area_calculator.area, 4.0 * 5.0);
}

#[test]
fn test_triangle_area() {
    let triangle = Triangle {
        base: 6.0,
        height: 7.0,
    };
    let mut area_calculator = AreaCalculator { area: 0.0 };

    triangle.accept(&mut area_calculator);

    assert_eq!(area_calculator.area, 0.5 * 6.0 * 7.0);
}
