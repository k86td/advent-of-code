use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}
fn part_1(input: &str) -> usize {
    0
}

fn part_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 1);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 1);
    }
}
