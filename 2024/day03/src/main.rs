use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let search =
        Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").expect("is valid matching regex");

    search
        .captures_iter(input)
        .map(|c| c.extract())
        .map(|(_, [a, b])| {
            a.parse::<usize>().expect("can parse A to number")
                * b.parse::<usize>().expect("can parse B to number")
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let input: String = input.lines().flat_map(|c| c.chars()).collect();

    let generic_search = Regex::new(r"do\(\)(.*?)(?:don't\(\)|$)").expect("valid regex");
    let search =
        Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").expect("is valid matching regex");

    let prepared_input = "do()".to_string() + &input;
    generic_search
        .find_iter(&prepared_input)
        .map(|c| c.as_str())
        .flat_map(|inner_search| {
            search
                .captures_iter(inner_search)
                .map(|c| c.extract())
                .map(|(_, [a, b])| {
                    a.parse::<usize>().expect("can parse A to number")
                        * b.parse::<usize>().expect("can parse B to number")
                })
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT_TEST_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 161);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST_2), 48);
    }
}
