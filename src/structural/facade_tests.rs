use crate::structural::facade::SmartHomeFacade;

#[test]
fn test_going_out_house() {
    let mut smart_home_facade = SmartHomeFacade::new();
    smart_home_facade.going_out(24.0);
    assert_eq!(smart_home_facade.security_system.locked, true)
}

#[test]
fn test_going_in_house() {
    let mut smart_home_facade = SmartHomeFacade::new();
    smart_home_facade.going_in(24.0);
    assert_eq!(smart_home_facade.security_system.locked, false)
}
