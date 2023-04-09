use crate::creational::factory::{AnimalFactory, AnimalType};

#[test]
fn dog_test() {
    assert_eq!("bow bow".to_string(), AnimalFactory::create_animal(AnimalType::Dog).noise());
}


#[test]
fn cat_test() {
    assert_eq!("meow meow".to_string(),  AnimalFactory::create_animal(AnimalType::Cat).noise());
}
