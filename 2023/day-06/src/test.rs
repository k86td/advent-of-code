use crate::{parse_input_multi, process_part_one, process_part_two, Race};

const TEST_RACE: &str = "
Time:      7  15   30
Distance:  9  40  200
";

#[test]
fn getting_races() {
    assert_eq!(
        parse_input_multi(TEST_RACE),
        vec![
            Race {
                distance: 9,
                time: 7
            },
            Race {
                time: 15,
                distance: 40
            },
            Race {
                time: 30,
                distance: 200
            }
        ]
    );
}

#[test]
fn get_wins() {
    assert_eq!(
        Race {
            time: 7,
            distance: 9
        }
        .get_wins(),
        vec![2, 3, 4, 5]
    );

    assert_eq!(
        Race {
            time: 15,
            distance: 40
        }
        .get_wins(),
        vec![4, 5, 6, 7, 8, 9, 10, 11]
    );
}

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_RACE), 288);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_RACE), 71503);
}

