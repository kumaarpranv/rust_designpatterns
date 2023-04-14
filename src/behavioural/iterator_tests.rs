use crate::behavioural::iterator::OddNumbers;

#[test]
fn odd_number_test() {
    let odd_numbers = OddNumbers::new(10);

    let collected_odd_numbers: Vec<u32> = odd_numbers.collect();
    let mut ct: u32 = 1;
    for i in collected_odd_numbers.iter() {
        assert_eq!(*i, ct);
        ct += 2;
    }
}
