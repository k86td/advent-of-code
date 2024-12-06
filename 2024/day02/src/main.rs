use std::fs;

fn parse(input: &str) -> Vec<Vec<isize>> {
    let mut result = Vec::new();
    for line in input.lines() {
        result.push(
            line.split_whitespace()
                .map(|d| d.parse().expect("can parse to usize"))
                .collect(),
        );
    }
    result
}

fn is_valid_report_levels(report: &[isize]) -> bool {
    let elem_diff = report
        .iter()
        .zip(report.iter().skip(1))
        .map(|(curr, next)| next - curr);

    // The levels are either all increasing or all decreasing
    let rule1: bool = {
        let (negative, positive): (Vec<isize>, Vec<isize>) =
            elem_diff.clone().partition(|e| e < &0);

        (!negative.is_empty() && positive.is_empty())
            || (negative.is_empty() && !positive.is_empty())
    };
    // Any two adjacent levels differ by at least one and at most three
    let rule2: bool = { elem_diff.clone().all(|e| (1..=3).contains(&e.abs())) };

    rule1 && rule2
}

fn part_1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .filter(|r| is_valid_report_levels(r))
        .count()
}

fn part_2(input: &str) -> usize {
    let mut correct_report_count: usize = 0;
    for report in parse(input) {
        let mut all_reports = vec![report.clone()]
            .into_iter()
            .chain((0..report.len()).map(|i| {
                let mut report = report.clone();
                report.remove(i);
                report
            }));

        if all_reports.any(|r| is_valid_report_levels(&r)) {
            correct_report_count += 1;
        }
    }

    correct_report_count
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[cfg(test)]
mod test {
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    use crate::{part_1, part_2};

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(TEST_INPUT), 2);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(TEST_INPUT), 4);
    }
}
