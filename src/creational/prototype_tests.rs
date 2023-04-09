use crate::creational::prototype::{Circle, Rectangle, Shape};

#[test]
fn circle_test() {
    let circle = Circle { radius: 5.0 };
    let circle_clone: Circle = circle.clone();
    assert_eq!(circle.area(), circle_clone.area());
}

#[test]
fn rectangle_test() {
    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    let rectangle_clone: Rectangle = rectangle.clone();
    assert_eq!(rectangle.area(), rectangle_clone.area());
}
