use crate::behavioural::strategy::{NoDiscount, PercentageDiscount, Product, FixedAmountDiscount};

#[test]
fn no_discount() {
    let product = Product::new(100.0, Box::new(NoDiscount));
    assert_eq!(product.price_with_discount(), 100.0);
}

#[test]
fn percentage_discount() {
    let product = Product::new(100.0, Box::new(PercentageDiscount(10.0)));
    assert_eq!(product.price_with_discount(), 90.0);
}

#[test]
fn fixed_amount_discount() {
    let product = Product::new(100.0, Box::new(FixedAmountDiscount(15.0)));
    assert_eq!(product.price_with_discount(), 85.0);
}
