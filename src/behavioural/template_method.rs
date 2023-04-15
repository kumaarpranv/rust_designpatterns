pub trait Beverage {
    fn prepare(&self) -> String {
        let mut result = String::new();
        result.push_str(&self.boil_water());
        result.push_str(&self.brew());
        result.push_str(&self.pour_in_cup());
        result.push_str(&self.add_condiments());
        result
    }

    fn boil_water(&self) -> String {
        String::from("Boiling water...\n")
    }

    fn brew(&self) -> String;
    fn pour_in_cup(&self) -> String {
        String::from("Pouring into cup...\n")
    }
    fn add_condiments(&self) -> String;
}

pub struct Coffee;

impl Beverage for Coffee {
    fn brew(&self) -> String {
        String::from("Brewing coffee...\n")
    }

    fn add_condiments(&self) -> String {
        String::from("Adding sugar and milk...\n")
    }
}

pub struct Tea;

impl Beverage for Tea {
    fn brew(&self) -> String {
        String::from("Steeping tea...\n")
    }

    fn add_condiments(&self) -> String {
        String::from("Adding lemon...\n")
    }
}
