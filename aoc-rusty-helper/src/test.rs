use crate::math::{factors_of, lcm_of};

#[test]
fn test_factors_of() {
    assert_eq!(factors_of(315), vec![3, 3, 5, 7]);
    assert_eq!(factors_of(12), vec![2, 2, 3]);
    assert_eq!(factors_of(4), vec![2, 2]);
    assert_eq!(factors_of(9), vec![3, 3]);
    assert_eq!(factors_of(42), vec![2, 3, 7]);
}

#[test]
fn test_lcm_of() {
    assert_eq!(lcm_of(&[12, 16, 24]), 48);
    assert_eq!(lcm_of(&[6, 7, 21]), 42);
}
