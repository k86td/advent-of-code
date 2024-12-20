use std::{collections::HashMap, fs, ops::Add};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

pub fn parse(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|d| d.parse().unwrap())
        .collect()
}

#[derive(Debug, Default, Clone, Copy)]
pub struct CacheStats {
    hit: usize,
    miss: usize,
    single: usize,
    split: usize,
}

impl Add for CacheStats {
    type Output = CacheStats;

    fn add(self, rhs: Self) -> Self::Output {
        CacheStats {
            hit: self.hit + rhs.hit,
            miss: self.miss + rhs.miss,
            single: self.single + rhs.single,
            split: self.split + rhs.split,
        }
    }
}

pub fn apply_rule(
    stone: usize,
    cache: &mut HashMap<usize, Vec<usize>>,
    stats: &mut CacheStats,
) -> Vec<usize> {
    if let Some(result) = cache.get(&stone) {
        stats.hit += 1;
        result.to_vec()
    } else if stone == 0 {
        stats.miss += 1;
        stats.single += 1;
        cache.insert(stone, [1].to_vec());
        vec![1]
    } else if stone.ilog10() % 2 == 1 {
        stats.miss += 1;
        stats.split += 1;
        cache.insert(stone, split_number(stone).to_vec());
        split_number(stone).to_vec()
    } else {
        stats.miss += 1;
        stats.single += 1;
        cache.insert(stone, [stone * 2024].to_vec());
        vec![stone * 2024]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuleResult {
    Single(usize),
    Split(usize, usize),
}

pub fn apply_rule_map(stone: usize, cache: &mut HashMap<usize, RuleResult>) -> RuleResult {
    if let Some(result) = cache.get(&stone) {
        *result
    } else if stone == 0 {
        cache.insert(stone, RuleResult::Single(1));
        RuleResult::Single(1)
    } else if stone.ilog10() % 2 == 1 {
        let [a, b] = split_number(stone);
        cache.insert(stone, RuleResult::Split(a, b));
        RuleResult::Split(a, b)
    } else {
        cache.insert(stone, RuleResult::Single(stone * 2024));
        RuleResult::Single(stone * 2024)
    }
}

pub const fn split_number(number: usize) -> [usize; 2] {
    if number < 10 {
        panic!("tried to split number smaller than 10");
    }

    let log = 10usize.pow(number.ilog10() / 2) * 10;

    [number / log, number % log]
}

fn part_1(input: &str) -> usize {
    let mut cache = HashMap::new();
    let mut stones = parse(input);
    let mut stats = CacheStats::default();

    const TOTAL: usize = 25;
    (0..TOTAL).for_each(|_| {
        stones = stones
            .iter_mut()
            .flat_map(|s| apply_rule(*s, &mut cache, &mut stats))
            .collect();
    });

    stones.len()
}

fn part_2(input: &str) -> usize {
    let mut stones: HashMap<usize, usize> =
        parse(input)
            .into_iter()
            .fold(HashMap::default(), |mut acc, s| {
                acc.insert(s, 1);
                acc
            });
    let mut cache: HashMap<usize, RuleResult> = HashMap::new();

    const TOTAL: usize = 75;
    (0..TOTAL).for_each(|_| {
        let mut new_stones: HashMap<usize, usize> = HashMap::new();

        for key in stones.keys() {
            if let Some(count) = stones.get(key) {
                match apply_rule_map(*key, &mut cache) {
                    RuleResult::Single(s) => {
                        *new_stones.entry(s).or_default() += *count;
                    }
                    RuleResult::Split(a, b) => {
                        *new_stones.entry(a).or_default() += *count;
                        *new_stones.entry(b).or_default() += *count;
                    }
                }
            }
        }

        stones = new_stones;
    });

    stones.into_values().sum::<usize>()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "125 17";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 55312);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 65601038650482);
    }
}
