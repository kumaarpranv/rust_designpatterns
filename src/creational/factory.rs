pub trait Animal {
    fn noise(&self) -> String;
}

pub struct Dog;
pub struct Cat;

impl Animal for Dog {
    fn noise(&self) -> String {
        "bow bow".to_string()
    }
}

impl Animal for Cat {
    fn noise(&self) -> String {
        "meow meow".to_string()
    }
}

pub enum AnimalType {
    Dog,
    Cat,
}

pub struct AnimalFactory;

impl AnimalFactory {
    pub fn create_animal(animal_type: AnimalType) -> Box<dyn Animal> {
        match animal_type {
            AnimalType::Dog => Box::new(Dog),
            AnimalType::Cat => Box::new(Cat),
        }
    }
}
