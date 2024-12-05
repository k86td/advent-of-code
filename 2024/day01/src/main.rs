use itertools::Itertools;
use std::{collections::HashMap, fs};

const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

pub fn part_1(input: String) -> usize {
    let mut la: Vec<isize> = Vec::new();
    let mut lb: Vec<isize> = Vec::new();

    for line in input.lines() {
        let mut elems = line.split_whitespace();
        la.push(elems.next().unwrap().parse().unwrap());
        lb.push(elems.next().unwrap().parse().unwrap());
    }

    la.sort();
    lb.sort();

    assert!(la.len() == lb.len());

    let mut result: Vec<isize> = Vec::new();
    for (a, b) in la.iter().zip(lb) {
        result.push((b - a).abs())
    }

    result.iter().sum::<isize>() as usize
}

fn group_occurences(elems: Vec<isize>) -> HashMap<isize, usize> {
    let mut result = HashMap::new();

    for (key, chunk) in &elems.into_iter().chunk_by(|e| *e) {
        result.insert(key, chunk.into_iter().count());
    }

    result
}

pub fn part_2(input: String) -> usize {
    let mut la: Vec<isize> = Vec::new();
    let mut lb: Vec<isize> = Vec::new();

    for line in input.lines() {
        let mut elems = line.split_whitespace();
        la.push(elems.next().unwrap().parse().unwrap());
        lb.push(elems.next().unwrap().parse().unwrap());
    }

    la.sort();
    lb.sort();

    assert!(la.len() == lb.len());

    let lb = group_occurences(lb);

    let mut result = 0;
    for elem in la {
        result += (elem as usize) * lb.get(&elem).unwrap_or(&0);
    }

    result
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read the input file");

    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
}

#[cfg(test)]
mod tests {
    use crate::TEST_INPUT;

    #[test]
    fn can_solve_part_1() {
        assert_eq!(crate::part_1(TEST_INPUT.to_string()), 11);
    }

    #[test]
    fn can_solve_part_2() {
        assert_eq!(crate::part_2(TEST_INPUT.to_string()), 31);
    }
}
