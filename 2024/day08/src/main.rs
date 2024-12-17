use std::{collections::HashSet, fs, panic::resume_unwind};

use nalgebra::Vector2;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

/// { x, y }
type Coordinate = Vector2<isize>;

#[derive(Debug, Clone)]
pub struct MapLocation {
    coord: Coordinate,
    element: MapElement,
}

#[derive(Debug, Clone)]
pub enum MapElement {
    Empty,
    Antenna(char),
}

pub fn parse(input: &str) -> Vec<MapLocation> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| MapLocation {
                    coord: Vector2::new(x as isize, y as isize),
                    element: match c {
                        'A'..='z' | '0'..='9' => MapElement::Antenna(c),
                        _ => MapElement::Empty,
                    },
                })
                .collect::<Vec<MapLocation>>()
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let grid = parse(input);
    let width = grid.last().unwrap().coord.x;
    let height = grid.last().unwrap().coord.y;

    let mut result: HashSet<Coordinate> = HashSet::new();

    grid.iter().for_each(
        |MapLocation {
             coord: coord_init,
             element: e_init,
         }| {
            if let MapElement::Antenna(co) = e_init {
                grid.iter()
                    .for_each(|MapLocation { coord, element }| match element {
                        MapElement::Antenna(ci) if ci == co && coord_init != coord => {
                            let m = coord_init - coord + coord_init;
                            if m.x <= width && m.x >= 0 && m.y <= height && m.y >= 0 {
                                result.insert(m);
                            }
                        }
                        _ => (),
                    })
            }
        },
    );
    // .flatten()
    // .filter(|m| m.x < width && m.x >= 0 && m.y < height && m.y >= 0)
    // .count()

    result.len()
}

fn part_2(input: &str) -> usize {
    let grid = parse(input);
    let width = grid.last().unwrap().coord.x;
    let height = grid.last().unwrap().coord.y;

    let mut result: HashSet<Coordinate> = HashSet::new();

    grid.iter().for_each(
        |MapLocation {
             coord: coord_init,
             element: e_init,
         }| {
            if let MapElement::Antenna(co) = e_init {
                grid.iter()
                    .for_each(|MapLocation { coord, element }| match element {
                        MapElement::Antenna(ci) if ci == co && coord_init != coord => {
                            let diff = coord_init - coord;
                            let mut m = diff + coord_init;
                            let mut count = 0;

                            // forward
                            while m.x <= width && m.x >= 0 && m.y <= height && m.y >= 0 {
                                result.insert(m);

                                m = diff + m;
                                count += 1;
                            }

                            //backward
                            let diff = diff * -1;
                            let mut m = diff + coord_init;
                            while m.x <= width && m.x >= 0 && m.y <= height && m.y >= 0 {
                                result.insert(m);

                                m = diff + m;
                                count += 1;
                            }

                            if count >= 2 {
                                result.insert(*coord_init);
                            }
                        }
                        _ => (),
                    })
            }
        },
    );

    result.len()
}

#[cfg(test)]
mod test {

    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 14);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 34);
    }
}
