use crate::behavioural::template_method::{Coffee, Tea, Beverage};

#[test]
fn test_coffee_tea() {
    let coffee = Coffee;
    let expected = "Boiling water...\nBrewing coffee...\nPouring into cup...\nAdding sugar and milk...\n";
    assert_eq!(coffee.prepare(), expected);

    let tea = Tea;
    let expected = "Boiling water...\nSteeping tea...\nPouring into cup...\nAdding lemon...\n";
    assert_eq!(tea.prepare(), expected);
}
