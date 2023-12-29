const TEST_INPUT: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

const SECOND_TEST_INPUT: &str = "
-50 -40 -30 -20 -10 0
0 -10 -20 -30 -40 -50
";

use crate::{parse, process_part_one, process_part_two, InternalSequence, Sequence};

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_INPUT), 114);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_INPUT), 2);
}

#[test]
fn test_internal_sequence() {
    let s1 = Sequence {
        values: vec![0, 3, 6, 9, 12, 15],
    };
    let s2 = Sequence {
        values: vec![3, 3, 3, 3, 3],
    };

    assert_eq!(s1.internal_sequence().unwrap(), s2);
    assert_eq!(s2.internal_sequence(), None);
}

#[test]
fn test_parse_sequence_from_str() {
    assert_eq!(
        Sequence::from("0 3 6 9 12 15"),
        Sequence {
            values: vec![0, 3, 6, 9, 12, 15]
        }
    );
    assert_eq!(
        Sequence::from("0 -1 -2 -3 -4 -5"),
        Sequence {
            values: vec![0, -1, -2, -3, -4, -5]
        }
    );
}

#[test]
fn test_getting_all_internal_sequences() {
    assert_eq!(
        Sequence::get_internal_sequences("0 3 6 9 12 15"),
        InternalSequence {
            sequences: vec![
                Sequence {
                    values: vec![0, 3, 6, 9, 12, 15]
                },
                Sequence {
                    values: vec![3, 3, 3, 3, 3]
                }
            ]
        }
    );
}

#[test]
fn test_internal_sequence_collapse() {
    assert_eq!(
        Sequence::get_internal_sequences("0 3 6 9 12 15").collapse(),
        18
    );
    assert_eq!(
        Sequence::get_internal_sequences("1 3 6 10 15 21").collapse(),
        28
    );
    assert_eq!(
        Sequence::get_internal_sequences("10 13 16 21 30 45").collapse(),
        68
    );
}
