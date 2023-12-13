use crate::check_number_with_symbol;
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
            vec![(1, 1)],
            vec![Number {
                value: 456,
                begin: (0, 0),
                end: (2, 0)
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
            vec![(1, 1)],
            vec![
                Number {
                    value: 456,
                    begin: (0, 0),
                    end: (2, 0)
                },
                Number {
                    value: 788,
                    begin: (4, 0),
                    end: (6, 0)
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

    assert!(check_number_with_symbol(symbols[0], numbers[0]));
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

