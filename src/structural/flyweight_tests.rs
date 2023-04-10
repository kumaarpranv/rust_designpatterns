use crate::structural::flyweight::{Forest, Height};

#[test]
fn test_tree_shorts() {
    let mut forest = Forest::new();
    forest.add_tree("red".to_string(), Height::Short);
    forest.add_tree("blue".to_string(), Height::Short);
    forest.add_tree("green".to_string(), Height::Tall);
    forest.add_tree("green".to_string(), Height::Medium);

    assert_eq!(forest.ret_all_short(), 2)
}
