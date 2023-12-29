const TEST_INPUT: &str = "
.....
.S-7.
.|.|.
.L-J.
.....
";

const SECOND_TEST_INPUT: &str = "
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

use crate::{get_connected, process_part_one, process_part_two, Map, Pipe, Pos};

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_INPUT), 4);
    assert_eq!(process_part_one(SECOND_TEST_INPUT), 8);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_INPUT), 0);
}

#[test]
fn getting_map() {
    let map = Map::from(TEST_INPUT);

    assert_eq!(
        map.start_loc,
        Pos {
            coord: (1, 1),
            pipe: Pipe::Start
        }
    );
}

#[test]
fn test_getting_connection() {
    let map = Map::from(TEST_INPUT);

    assert_eq!(
        get_connected(&map, &map.start_loc),
        [
            Pos {
                coord: (2, 1),
                pipe: Pipe::LeftRight
            },
            Pos {
                coord: (1, 2),
                pipe: Pipe::TopDown
            },
        ]
    );
}

#[test]
fn test_connected_pipes() {
    let p1 = Pos {
        coord: (5, 5),
        pipe: Pipe::TopRight,
    };
    let p2 = Pos {
        coord: (6, 5),
        pipe: Pipe::DownLeft,
    };

    assert!(p1.is_connected(&p2));
}

#[test]
fn test_moving_map() {
    let map = Map::from(TEST_INPUT);

    assert_eq!(
        map.left(&map.start_loc),
        Some(&Pos {
            coord: (0, 1),
            pipe: Pipe::Ground
        })
    );
}
