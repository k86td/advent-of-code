use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    fs, usize,
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

// could experiment with discoverable 2D grid, meaning we're building the vec as we try to see it.
// could save some memory but would take more time
pub fn parse(input: &str) -> HashMap<(usize, usize), u8> {
    let mut result = HashMap::new();

    let lines = input.trim().lines();

    lines.into_iter().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, tile)| {
            result.insert((x, y), tile.to_digit(10).unwrap() as u8);
        })
    });
    result
}

pub fn get_cardinals(
    map: &HashMap<(usize, usize), u8>,
    x: usize,
    y: usize,
) -> [Option<(&(usize, usize), &u8)>; 4] {
    [
        map.get_key_value(&(x, y.saturating_sub(1))),
        map.get_key_value(&(x, y.saturating_add(1))),
        map.get_key_value(&(x.saturating_add(1), y)),
        map.get_key_value(&(x.saturating_sub(1), y)),
    ]
}

fn part_1(input: &str) -> usize {
    let map = parse(input);

    let mut reached_top = 0;
    let mut to_hike: Vec<((usize, usize), u8)> = Vec::with_capacity(100);

    map.iter().filter(|(_, &v)| v == 0).for_each(|(c, t)| {
        to_hike.push(((c.0, c.1), *t));
    });

    to_hike.into_iter().for_each(|((x, y), tile)| {
        let mut trail_end: HashSet<(usize, usize)> = HashSet::new();

        let mut path: Vec<((usize, usize), u8)> = vec![((x, y), tile)];
        while let Some(((x, y), tile)) = path.pop() {
            if tile == 9 {
                trail_end.insert((x, y));
                continue;
            }

            get_cardinals(&map, x, y).into_iter().for_each(|d| {
                if let Some(((xi, yi), inner_tile)) = d {
                    if *inner_tile == tile + 1 {
                        path.push(((*xi, *yi), *inner_tile));
                    }
                }
            })
        }

        reached_top += trail_end.len();
    });

    reached_top
}

fn part_2(input: &str) -> usize {
    let map = parse(input);

    let mut reached_top = 0;
    let mut to_hike: Vec<((usize, usize), u8)> = Vec::with_capacity(100);

    map.iter().filter(|(_, &v)| v == 0).for_each(|(c, t)| {
        to_hike.push(((c.0, c.1), *t));
    });

    to_hike.into_iter().for_each(|((x, y), tile)| {
        let mut trail_end: Vec<(usize, usize)> = Vec::new();

        let mut path: Vec<((usize, usize), u8)> = vec![((x, y), tile)];
        while let Some(((x, y), tile)) = path.pop() {
            if tile == 9 {
                trail_end.push((x, y));
                continue;
            }

            get_cardinals(&map, x, y).into_iter().for_each(|d| {
                if let Some(((xi, yi), inner_tile)) = d {
                    if *inner_tile == tile + 1 {
                        path.push(((*xi, *yi), *inner_tile));
                    }
                }
            })
        }

        reached_top += trail_end.len();
    });

    reached_top
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 36);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 81);
    }
}
