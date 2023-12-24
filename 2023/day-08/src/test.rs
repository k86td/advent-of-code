const TEST_INPUT: &str = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

const SECOND_TEST_INPUT: &str = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

use std::collections::HashMap;

use crate::{parse, process_part_one, process_part_two, Directions, Game, Node};

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_INPUT), 2);
    assert_eq!(process_part_one(SECOND_TEST_INPUT), 6);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_INPUT), 0);
}

#[test]
fn parsing_node_from_string() {
    assert_eq!(
        Node::from("(BBB, CCC)"),
        Node {
            left: "BBB".to_string(),
            right: "CCC".to_string()
        }
    );
}

#[test]
fn parsing_from_test_input() {
    let mut corr_map: HashMap<String, Node> = HashMap::new();

    corr_map.insert(
        "AAA".to_string(),
        Node {
            left: "BBB".to_string(),
            right: "CCC".to_string(),
        },
    );
    corr_map.insert(
        "BBB".to_string(),
        Node {
            left: "DDD".to_string(),
            right: "EEE".to_string(),
        },
    );
    corr_map.insert(
        "CCC".to_string(),
        Node {
            left: "ZZZ".to_string(),
            right: "GGG".to_string(),
        },
    );
    corr_map.insert(
        "DDD".to_string(),
        Node {
            left: "DDD".to_string(),
            right: "DDD".to_string(),
        },
    );
    corr_map.insert(
        "EEE".to_string(),
        Node {
            left: "EEE".to_string(),
            right: "EEE".to_string(),
        },
    );
    corr_map.insert(
        "GGG".to_string(),
        Node {
            left: "GGG".to_string(),
            right: "GGG".to_string(),
        },
    );
    corr_map.insert(
        "ZZZ".to_string(),
        Node {
            left: "ZZZ".to_string(),
            right: "ZZZ".to_string(),
        },
    );

    assert_eq!(
        parse(TEST_INPUT),
        Game {
            map: corr_map,
            direction: vec![Directions::Right, Directions::Left]
        }
    );
}
