use crate::structural::decorator::{BeverageDecorator, Beverage, Coffee, Milk, Sugar};

#[test]
fn test_coffee_milk_sugar() {
    let coffee = Coffee::new();
    let coffee_with_milk: Box<dyn Beverage> = Box::new(Milk::new(Box::new(coffee)));
    let coffee_with_milk_and_sugar: Box<dyn Beverage> = Box::new(Sugar::new(coffee_with_milk));
    assert_eq!("Coffee with Milk with Sugar", coffee_with_milk_and_sugar.description());
}
