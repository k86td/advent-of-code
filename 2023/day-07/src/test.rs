const TEST_INPUT: &str = "";

use crate::{process_part_one, process_part_two, Card};

#[test]
fn card_compare() {
    assert!(Card::A > Card::K);
    assert!(Card::J > Card::Number(10));
    assert!(Card::Number(10) > Card::Number(2));
}

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_INPUT), 0);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_INPUT), 0);
}

