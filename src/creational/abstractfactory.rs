pub trait Mammal {
    fn noise(&self) -> String;
}

pub trait Bird {
    fn noise(&self) -> String;
}

pub enum MammalType {
    Dog,
    Cat,
}

pub enum BirdType {
    Eagle,
    Parrot,
}

pub trait MammalFactory {
    fn create_mammal(&self, mammal_type: MammalType) -> Box<dyn Mammal>;
}

pub trait BirdFactory {
    fn create_bird(&self, bird_type: BirdType) -> Box<dyn Bird>;
}

pub struct Dog;
pub struct Cat;
pub struct Eagle;
pub struct Parrot;

impl Mammal for Dog {
    fn noise(&self) -> String {
        "bow bow".to_string()
    }
}

impl Mammal for Cat {
    fn noise(&self) -> String {
        "meow meow".to_string()
    }
}

impl Bird for Eagle {
    fn noise(&self) -> String {
        "screech".to_string()
    }
}

impl Bird for Parrot {
    fn noise(&self) -> String {
        "squawk".to_string()
    }
}

pub struct ZooFactory;

impl MammalFactory for ZooFactory {

    fn create_mammal(&self, mammal_type: MammalType) -> Box<dyn Mammal> {
        match mammal_type {
            MammalType::Dog => Box::new(Dog),
            MammalType::Cat => Box::new(Cat),
        }
    }
}

impl BirdFactory for ZooFactory {

    fn create_bird(&self, bird_type: BirdType) -> Box<dyn Bird> {
        match bird_type {
            BirdType::Eagle => Box::new(Eagle),
            BirdType::Parrot => Box::new(Parrot),
        }
    }
}
