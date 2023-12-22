use crate::{
    get_seed_parts, parse_input, process_input, process_part_one, process_part_two, test,
    ItemToItemRange, MapRange, OptRangeMap, SeedPart,
};

const test_input: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

#[test]
fn part_1() {
    assert_eq!(process_part_one(test_input), 35);
}

#[test]
fn part_1_big() {
    let file = include_str!("../input");

    let file_content =
        std::fs::read_to_string("C:/Users/tlepine/source/repos/advent-of-code/2023/day-05/input")
            .unwrap();

    process_part_one(&file_content);
}

#[test]
fn getting_seed_parts() {
    let mut lines = test_input.split("\n\n");

    _ = lines.nth(0);

    let seeds = vec![79, 14, 55, 13];
    let maps = process_input(lines.collect());

    let seed1 = seeds.get(0).unwrap();
    let seed2 = seeds.get(1).unwrap();
    let seed3 = seeds.get(2).unwrap();
    let seed4 = seeds.get(3).unwrap();

    assert_eq!(
        get_seed_parts(*seed1, maps.clone()),
        SeedPart {
            seed: 79,
            soil: 81,
            fertilizer: 81,
            water: 81,
            light: 74,
            temperature: 78,
            humidity: 78,
            location: 82
        }
    );

    assert_eq!(
        get_seed_parts(*seed2, maps.clone()),
        SeedPart {
            seed: 14,
            soil: 14,
            fertilizer: 53,
            water: 49,
            light: 42,
            temperature: 42,
            humidity: 43,
            location: 43
        }
    );

    assert_eq!(
        get_seed_parts(*seed3, maps.clone()),
        SeedPart {
            seed: 55,
            soil: 57,
            fertilizer: 57,
            water: 53,
            light: 46,
            temperature: 82,
            humidity: 82,
            location: 86
        }
    );
}

#[test]
fn convert_to_maprange() {
    let input = "50 98 2";

    let corr_output = MapRange {
        destination: 50..52,
        source: 98..100,
    };
    let curr_output: MapRange = input.into();

    assert_eq!(curr_output, corr_output);
}

#[test]
fn convert_to_itemtoitemrange() {
    let input = "
    seed-to-soil map:
    50 98 2
    52 50 48
    ";

    let corr_output = ItemToItemRange {
        name: "seed-to-soil".to_string(),
        maps: vec![
            MapRange {
                destination: 52..52 + 48,
                source: 50..50 + 48,
            },
            MapRange {
                destination: 50..52,
                source: 98..100,
            },
        ],
    };
    let curr_output: ItemToItemRange = input.into();

    assert_eq!(curr_output, corr_output);
}

#[test]
fn get_correct_range_mapping() {
    let map_range = MapRange {
        source: 50..55,
        destination: 10..15,
    };

    let map_range2 = MapRange {
        source: 10..15,
        destination: 230..235,
    };

    assert_eq!(map_range.get_mapped(50).unwrap(), 10);
    assert_eq!(map_range.get_mapped(10), None);
    assert_eq!(map_range2.get_mapped(230), None);
    assert_eq!(map_range2.get_mapped(11).unwrap(), 231);
}

#[test]
fn get_correct_multiple_range_mapping() {
    let m1 = MapRange {
        source: 50..60,
        destination: 200..210,
    };
    let m2 = MapRange {
        source: 10..15,
        destination: 230..235,
    };
    let m3 = MapRange {
        source: 15..20,
        destination: 0..5,
    };

    let i2i_range = ItemToItemRange {
        name: "water-to-light".to_string(),
        maps: vec![m1, m2, m3],
    };

    assert_eq!(i2i_range.get_mapped(1), 1);
    assert_eq!(i2i_range.get_mapped(50), 200);
    assert_eq!(i2i_range.get_mapped(15), 0);
    assert_eq!(i2i_range.get_mapped(500), 500);
    assert_eq!(i2i_range.get_mapped(230), 230);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(test_input), 46);
}
