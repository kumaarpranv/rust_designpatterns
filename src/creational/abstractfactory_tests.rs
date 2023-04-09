use crate::creational::abstractfactory::{ZooFactory, MammalFactory, BirdFactory, MammalType, BirdType};

#[test]
fn dog_test() {
    let factory = ZooFactory;
    assert_eq!("bow bow".to_string(), factory.create_mammal(MammalType::Dog).noise());
}


#[test]
fn cat_test() {
    let factory = ZooFactory;
    assert_eq!("meow meow".to_string(),  factory.create_mammal(MammalType::Cat).noise());
}


#[test]
fn eagle_test() {
    let factory = ZooFactory;
    assert_eq!("screech".to_string(),  factory.create_bird(BirdType::Eagle).noise());
}

#[test]
fn parrot_test() {
    let factory = ZooFactory;
    assert_eq!("squawk".to_string(),  factory.create_bird(BirdType::Parrot).noise());
}
