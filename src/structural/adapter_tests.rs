use crate::structural::adapter::{
    ConcreteRoundHole, ConcreteRoundPeg, ConcreteSquarePeg, SquarePegAdapter,
};

#[test]
fn test_adapter_plug_fits() {
    let square_peg_small = ConcreteSquarePeg::new(5.0);
    let square_peg_adapter = SquarePegAdapter::new(square_peg_small);
    let round_hole = ConcreteRoundHole::new(6.0);
    assert_eq!(true, round_hole.fits(&square_peg_adapter));
}

#[test]
fn test_adapter_plug_not_fits() {
    let square_peg_big = ConcreteSquarePeg::new(18.0);
    let square_peg_adapter = SquarePegAdapter::new(square_peg_big);
    let round_hole = ConcreteRoundHole::new(6.0);
    assert_eq!(false, round_hole.fits(&square_peg_adapter));
}

#[test]
fn test_round_plug_fits() {
    let round_peg_small = ConcreteRoundPeg::new(5.0);
    let round_hole = ConcreteRoundHole::new(6.0);
    assert_eq!(true, round_hole.fits(&round_peg_small));
}

#[test]
fn test_round_plug_not_fits() {
    let round_peg_big = ConcreteRoundPeg::new(18.0);
    let round_hole = ConcreteRoundHole::new(6.0);
    assert_eq!(false, round_hole.fits(&round_peg_big));
}
