use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Debug, Clone)]
pub struct Equation {
    supposed_result: usize,
    numbers: Vec<usize>,
}

impl Equation {
    pub fn is_valid_pt1(&self, value: usize, index: usize) -> bool {
        match self.numbers.get(index + 1) {
            Some(next_value) => {
                if value > self.supposed_result {
                    false
                } else {
                    let index = index + 1;
                    self.is_valid_pt1(value + next_value, index)
                        || self.is_valid_pt1(value * next_value, index)
                }
            }
            None => value == self.supposed_result,
        }
    }

    pub fn is_valid_pt2(&self, value: usize, index: usize) -> bool {
        match self.numbers.get(index + 1) {
            Some(next_value) => {
                if value > self.supposed_result {
                    false
                } else {
                    let index = index + 1;
                    self.is_valid_pt2(value + next_value, index)
                        || self.is_valid_pt2(value * next_value, index)
                        || self.is_valid_pt2(
                            format!("{}{}", value, next_value).parse().unwrap(),
                            index,
                        )
                }
            }
            None => value == self.supposed_result,
        }
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splitted = s.split(':');
        Ok(Self {
            supposed_result: splitted
                .next()
                .expect("can get the first element")
                .parse()
                .expect("can parse total to usize"),
            numbers: splitted
                .next()
                .expect("can get numbers")
                .split_whitespace()
                .flat_map(|c| c.parse())
                .collect(),
        })
    }
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            if let Ok(e) = l.parse::<Equation>() {
                match e.is_valid_pt1(e.numbers[0], 0) {
                    true => Some(e.supposed_result),
                    false => None,
                }
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|l| {
            if let Ok(e) = l.parse::<Equation>() {
                match e.is_valid_pt2(e.numbers[0], 0) {
                    true => Some(e.supposed_result),
                    false => None,
                }
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 3749);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 11387);
    }
}
