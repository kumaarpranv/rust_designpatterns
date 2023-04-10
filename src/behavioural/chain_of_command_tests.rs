use crate::behavioural::chain_of_command::{Department, Reception, Doctor, Cashier};

#[test]
fn test_doctor_chain() {
    let mut reception = Reception::new();
    let mut doctor = Doctor::new();
    let cashier = Cashier::new();

    doctor.set_next(Some(Box::new(cashier)));
    reception.set_next(Some(Box::new(doctor)));
    reception.handle_request("patient_request");
}
