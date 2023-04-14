use crate::behavioural::command::{Command, Calculator, AddCommand, SubtractCommand, MultiplyCommand, DivideCommand};

#[test]
fn test_calculator_flow() {
    let calculator = Calculator { value: 10.0 };

    let add_command = AddCommand::new(
        calculator.clone(),
        6.0,
    );

    let subtract_command = SubtractCommand::new(
        calculator.clone(),
        5.0,
    );

    let multiply_command = MultiplyCommand::new(
        calculator.clone(),
        2.0,
    );

    let divide_command = DivideCommand::new(
        calculator.clone(),
        2.0,
    );

    assert_eq!(16, add_command.execute() as u64); 
    assert_eq!(5, subtract_command.execute() as u64);
    assert_eq!(20, multiply_command.execute() as u64);
    assert_eq!(5, divide_command.execute() as u64);
}
