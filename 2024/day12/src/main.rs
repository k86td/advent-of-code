use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    fs,
    ops::Add,
    thread::LocalKey,
    usize,
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub enum Tile {
    /// Represent the number of sides this edge has
    Edge(usize),
    Middle,
}

#[derive(Debug)]
pub struct Zone {
    plant: char,
    tiles: Vec<Tile>,
    area: usize,
    perimeter: usize,
}

impl Zone {
    pub fn new(plant: char, tiles: Vec<Tile>) -> Self {
        Self {
            plant,
            area: Zone::area(&tiles),
            perimeter: Zone::perimeter(&tiles),
            tiles,
        }
    }

    fn area(tiles: &[Tile]) -> usize {
        tiles.len()
    }

    fn perimeter(tiles: &[Tile]) -> usize {
        tiles.iter().fold(0, |acc, s| match s {
            Tile::Edge(e) => acc + e,
            Tile::Middle => acc,
        })
    }

    pub fn only_corners(&mut self) {
        self.tiles.retain(|t| match t {
            Tile::Edge(s) => s >= &2,
            Tile::Middle => false,
        });
    }
}

pub fn parse(input: &str) -> HashMap<Coordinate, char> {
    let mut result = HashMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            result.insert(
                Coordinate {
                    x: x as isize,
                    y: y as isize,
                },
                c,
            );
        });
    });

    result
}

pub fn get_zone(
    location: Coordinate,
    map: &HashMap<Coordinate, char>,
    checked: &mut HashSet<Coordinate>,
) -> Zone {
    let our_char = map.get(&location).expect("can get from location");
    let mut tiles: Vec<Tile> = Vec::new();

    let mut to_check: Vec<Coordinate> = vec![location];
    while let Some(nxt) = to_check.pop() {
        if checked.contains(&nxt) {
            continue;
        }

        let mut edges = 0;

        [nxt.up(), nxt.right(), nxt.down(), nxt.left()]
            .map(|l| map.get_key_value(&l))
            .into_iter()
            .for_each(|o| {
                if let Some((ccoord, cchar)) = o {
                    if cchar == our_char {
                        if !checked.contains(ccoord) {
                            to_check.push(*ccoord);
                        }
                    } else {
                        edges += 1;
                    }
                } else {
                    edges += 1;
                }
            });

        checked.insert(nxt);

        if edges == 4 && !to_check.is_empty() {
            panic!("edges all sides but still tiles to check");
        } else if edges > 0 {
            tiles.push(Tile::Edge(edges));
        } else {
            tiles.push(Tile::Middle);
        }
    }

    Zone::new(*our_char, tiles)
}

fn part_1(input: &str) -> usize {
    let map = parse(input);
    let mut checked: HashSet<Coordinate> = HashSet::new();
    let mut zones: Vec<Zone> = Vec::new();

    for coord in map.clone().into_keys() {
        if checked.contains(&coord) {
            continue;
        }

        zones.push(get_zone(coord, &map, &mut checked));
    }

    zones.into_iter().map(|z| z.area * z.perimeter).sum()
}

fn part_2(input: &str) -> usize {
    let map = parse(input);
    let mut checked: HashSet<Coordinate> = HashSet::new();
    let mut zones: Vec<Zone> = Vec::new();

    for coord in map.clone().into_keys() {
        if checked.contains(&coord) {
            continue;
        }

        let mut z = get_zone(coord, &map, &mut checked);
        z.only_corners();

        zones.push(z);
    }

    zones
        .into_iter()
        .inspect(|z| {
            dbg!((z, Zone::perimeter(&z.tiles) / 2));
        })
        .map(|z| z.area * (Zone::perimeter(&z.tiles) / 2))
        .sum()
}

// could use the shoelace formula to calculate area & perimeter straight-forward way
// check https://www.omnicalculator.com/math/irregular-polygon-area

#[cfg(test)]
mod test {
    use crate::{get_zone, parse, part_1, part_2, Coordinate, Tile, Zone};

    const INPUT_TEST: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const INPUT_TEST2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST2), 772);
        assert_eq!(part_1(INPUT_TEST), 1930);
    }

    const INPUT_TEST3: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA";

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 1206);
        assert_eq!(part_2(INPUT_TEST2), 436);
    }
}
