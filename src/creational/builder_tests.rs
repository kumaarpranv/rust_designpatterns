use crate::creational::builder::{CarBuilder, CarDirector};

#[test]
fn sedan_test() {
    let mut builder = CarBuilder::new();
    let sedan =  CarDirector::construct_sedan(&mut builder).unwrap();
    assert_eq!("Toyota", sedan.make);
}

#[test]
fn sports_car_test() {
    let mut builder = CarBuilder::new();
    let sedan =  CarDirector::construct_sports_car(&mut builder).unwrap();
    assert_eq!("Porsche", sedan.make);
}
