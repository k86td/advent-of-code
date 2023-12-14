use crate::{number_edge_of_symbol, process_part_two, Coord, Symbol};
#[cfg(test)]
use crate::{parse_input, process_part_one, Number};

#[test]
fn parsing_single_number() {
    let input = "456
        .&.
        ...";

    assert_eq!(
        parse_input(input),
        (
            vec![Symbol {
                value: "&".to_string(),
                coord: Coord(1, 1)
            }],
            vec![Number {
                value: 456,
                begin: Coord(0, 0),
                end: Coord(2, 0)
            }]
        )
    );
}

#[test]
fn parsing_dual_number_same_line() {
    let input = "
        456.788
        .&.....
        .......
        ";

    assert_eq!(
        parse_input(input),
        (
            vec![Symbol {
                value: "&".to_string(),
                coord: Coord(1, 1)
            }],
            vec![
                Number {
                    value: 456,
                    begin: Coord(0, 0),
                    end: Coord(2, 0)
                },
                Number {
                    value: 788,
                    begin: Coord(4, 0),
                    end: Coord(6, 0)
                }
            ]
        )
    );
}

#[test]
fn symbol_detect_edge_number() {
    let input = "
    67.
    ..*";

    let (symbols, numbers) = parse_input(input);

    assert_eq!(symbols.len(), 1);
    assert_eq!(numbers.len(), 1);

    assert!(number_edge_of_symbol(symbols[0].clone(), numbers[0]));

    let input = "
    67..
    ...*";

    let (symbols, numbers) = parse_input(input);

    assert_eq!(symbols.len(), 1);
    assert_eq!(numbers.len(), 1);

    assert!(!number_edge_of_symbol(symbols[0].clone(), numbers[0]));
}

#[test]
fn point_in_rectangle() {
    assert!(number_edge_of_symbol(
        Symbol {
            value: "*".to_string(),
            coord: Coord(1, 1)
        },
        Number {
            value: 5,
            begin: Coord(0, 0),
            end: Coord(1, 0)
        }
    ));

    assert!(!number_edge_of_symbol(
        Symbol {
            value: "*".to_string(),
            coord: Coord(5, 1)
        },
        Number {
            value: 5,
            begin: Coord(0, 0),
            end: Coord(1, 0)
        }
    ));

    assert!(!number_edge_of_symbol(
        Symbol {
            value: "*".to_string(),
            coord: Coord(5, 2)
        },
        Number {
            value: 5,
            begin: Coord(7, 3),
            end: Coord(7, 3)
        }
    ));
}

#[test]
fn part_1() {
    let input = "
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
        ";

    assert_eq!(process_part_one(input), 4361);
}

#[test]
fn part_2() {
    let input = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    assert_eq!(process_part_two(input), 467835);
}
