use std::{
    cmp::{self, Ordering},
    fmt::Display,
    fs,
    io::{stdout, Write},
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Coordinate = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    pub fn rotate_cw(&self) -> Self {
        match self {
            Facing::Up => Self::Right,
            Facing::Right => Self::Down,
            Facing::Down => Self::Left,
            Facing::Left => Self::Up,
        }
    }

    pub fn forward(&self, location: Coordinate) -> Coordinate {
        match self {
            Facing::Up => (location.0, location.1 - 1),
            Facing::Down => (location.0, location.1 + 1),
            Facing::Left => (location.0 - 1, location.1),
            Facing::Right => (location.0 + 1, location.1),
        }
    }

    pub fn backward(&self, location: Coordinate) -> Coordinate {
        match self {
            Facing::Up => (location.0, location.1 + 1),
            Facing::Down => (location.0, location.1 - 1),
            Facing::Left => (location.0 + 1, location.1),
            Facing::Right => (location.0 - 1, location.1),
        }
    }

    pub fn apply(
        &self,
        location: &Coordinate,
        map: &MapGrid,
    ) -> Option<Vec<(Coordinate, MapElement)>> {
        let vertical = map.get_column(location.0).into_iter().flat_map(|col| {
            col.into_iter()
                .enumerate()
                .map(|(i, c)| ((location.0, i), c))
                .collect::<Vec<(Coordinate, MapElement)>>()
        });
        let horizontal = map.get_row(location.1).into_iter().flat_map(|col| {
            col.into_iter()
                .enumerate()
                .map(|(i, c)| ((i, location.1), c))
                .collect::<Vec<(Coordinate, MapElement)>>()
        });

        let result: Vec<(Coordinate, MapElement)> = match self {
            Facing::Up => vertical
                .filter(|((_, y), _)| y < &location.1)
                .rev()
                .collect(),
            Facing::Down => vertical.filter(|((_, y), _)| y > &location.1).collect(),
            Facing::Left => horizontal
                .filter(|((x, _), _)| x < &location.0)
                .rev()
                .collect(),
            Facing::Right => horizontal.filter(|((x, _), _)| x > &location.0).collect(),
        };

        if !result.is_empty() {
            Some(result)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MapElement {
    Empty,
    Obstacle,
    Guard { face: Facing },
}

impl Display for MapElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapElement::Empty => f.write_str("."),
            MapElement::Obstacle => f.write_str("#"),
            MapElement::Guard { face: _ } => f.write_str("^"),
        }
    }
}

impl MapElement {
    fn try_parse(content: &str) -> Result<Vec<Vec<MapElement>>, ()> {
        Ok(content
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => MapElement::Empty,
                        '#' => MapElement::Obstacle,
                        'v' => MapElement::Guard { face: Facing::Down },
                        '^' => MapElement::Guard { face: Facing::Up },
                        '>' => MapElement::Guard {
                            face: Facing::Right,
                        },
                        '<' => MapElement::Guard { face: Facing::Left },
                        _ => panic!("couldn't find character for parsing: '{}'", c),
                    })
                    .collect()
            })
            .collect())
    }
}

#[derive(Debug, Clone)]
struct MapGrid {
    elements: Vec<Vec<MapElement>>,
}

impl MapGrid {
    pub fn find_anywhere(&self, target: &MapElement) -> Option<Coordinate> {
        self.elements
            .iter()
            .enumerate()
            .filter_map(|(y, inner)| inner.iter().position(|e| e == target).map(|x| (x, y)))
            .next()
    }

    pub fn height(&self) -> usize {
        self.elements.len()
    }

    pub fn width(&self) -> usize {
        self.elements.first().expect("can get first element").len()
    }

    pub fn get(&self, location: &Coordinate) -> Option<MapElement> {
        self.elements
            .get(location.1)
            .and_then(|row| row.get(location.0).cloned())
    }

    pub fn set(&mut self, location: &Coordinate, element: MapElement) {
        self.elements
            .get_mut(location.1)
            .and_then(|row| row.get_mut(location.0).map(|i| *i = element));
    }

    pub fn get_row(&self, row: usize) -> Option<Vec<MapElement>> {
        self.elements.get(row).cloned()
    }

    pub fn get_column(&self, column: usize) -> Option<Vec<MapElement>> {
        let column: Vec<MapElement> = self
            .elements
            .iter()
            .filter_map(|c| c.get(column).cloned())
            .collect();

        if !column.is_empty() {
            Some(column)
        } else {
            None
        }
    }

    pub fn nearest_edge(&self, source: &Coordinate, facing: Facing) -> Option<Coordinate> {
        match facing {
            Facing::Up => Some((source.0, 0)),
            Facing::Down => Some((source.0, self.height() - 1)),
            Facing::Left => Some((0, source.1)),
            Facing::Right => Some((0, self.width() - 1)),
        }
    }

    pub fn find_facing(
        &self,
        source: &Coordinate,
        target: &MapElement,
        facing: Facing,
    ) -> Option<Coordinate> {
        facing.apply(source, self).map(|f| {
            f.into_iter()
                .filter_map(|(c, e)| {
                    if &e == target {
                        Some(facing.backward(c))
                    } else {
                        None
                    }
                })
                .next()
        })?
    }
}

fn coordinate_abs_distance(a: Coordinate, b: Coordinate) -> usize {
    (b.0 as isize - a.0 as isize).unsigned_abs() + (b.1 as isize - a.1 as isize).unsigned_abs()
}

fn coordinates_between(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    if (a.0 as isize - b.0 as isize).unsigned_abs() > (a.1 as isize - b.1 as isize).unsigned_abs() {
        // moving horizontally
        (cmp::min(a.0, b.0)..=cmp::max(a.0, b.0))
            .map(|i| (i, a.1))
            .collect()
    } else {
        // moving vertically
        (cmp::min(a.1, b.1)..=cmp::max(a.1, b.1))
            .map(|i| (a.0, i))
            .collect()
    }
}

fn coordinates_inbetween(a: Coordinate, b: Coordinate) -> Vec<Coordinate> {
    if (a.0 as isize - b.0 as isize).unsigned_abs() > (a.1 as isize - b.1 as isize).unsigned_abs() {
        // moving horizontally
        (cmp::min(a.0, b.0)..cmp::max(a.0, b.0))
            .skip(1)
            .map(|i| (i, a.1))
            .collect()
    } else {
        // moving vertically
        (cmp::min(a.1, b.1)..cmp::max(a.1, b.1))
            .skip(1)
            .map(|i| (a.0, i))
            .collect()
    }
}

fn part_1(input: &str) -> usize {
    let map = MapGrid {
        elements: MapElement::try_parse(input).expect("can parse input"),
    };

    let guard = map
        .find_anywhere(&MapElement::Guard { face: Facing::Up })
        .expect("can found guard");

    let mut distance: Vec<Coordinate> = Vec::new();
    let mut finding = (guard, Facing::Up);
    while let Some(found) = map.find_facing(&finding.0, &MapElement::Obstacle, finding.1) {
        distance.append(&mut coordinates_between(finding.0, found));
        finding = (found, finding.1.rotate_cw());
    }

    // this is ugly, should have something like map.get_edge(Coordinate, Facing)
    let mut empty = match finding.1 {
        Facing::Up => coordinates_between((finding.0 .0, 0), (finding.0 .0, finding.0 .1)),
        Facing::Down => coordinates_between(
            (finding.0 .0, finding.0 .1),
            (finding.0 .0, map.height() - 1),
        ),
        Facing::Left => coordinates_between((0, finding.0 .1), (finding.0 .0, finding.0 .1)),
        Facing::Right => coordinates_between(
            (finding.0 .0, finding.0 .1),
            (map.width() - 1, finding.0 .1),
        ),
    };

    distance.append(&mut empty);

    distance.sort();
    distance.dedup();

    distance.len()
}

fn part_2(input: &str) -> usize {
    let map = MapGrid {
        elements: MapElement::try_parse(input).expect("can parse input"),
    };

    let guard_start = map
        .find_anywhere(&MapElement::Guard { face: Facing::Up })
        .expect("can found guard");

    // Get all positions the guard visits in the original path
    let mut visited_positions = std::collections::HashSet::new();
    let mut current_pos = guard_start;
    let mut current_facing = Facing::Up;
    
    loop {
        visited_positions.insert(current_pos);
        
        if let Some(obstacle_pos) = map.find_facing(&current_pos, &MapElement::Obstacle, current_facing) {
            // Add all positions between current and obstacle
            for pos in coordinates_between(current_pos, obstacle_pos) {
                visited_positions.insert(pos);
            }
            current_pos = obstacle_pos;
            current_facing = current_facing.rotate_cw();
        } else {
            // Add positions to edge
            let edge_pos = match current_facing {
                Facing::Up => (current_pos.0, 0),
                Facing::Down => (current_pos.0, map.height() - 1),
                Facing::Left => (0, current_pos.1),
                Facing::Right => (map.width() - 1, current_pos.1),
            };
            for pos in coordinates_between(current_pos, edge_pos) {
                visited_positions.insert(pos);
            }
            break;
        }
    }

    // Try placing obstacle at each visited position (except guard start)
    let mut loop_count = 0;
    for &obstacle_pos in &visited_positions {
        if obstacle_pos == guard_start {
            continue; // Can't place obstacle at guard's starting position
        }

        // Create map with new obstacle
        let mut test_map = map.clone();
        test_map.set(&obstacle_pos, MapElement::Obstacle);

        // Simulate guard movement and check for loops
        let mut visited_states = std::collections::HashSet::new();
        let mut current_pos = guard_start;
        let mut current_facing = Facing::Up;

        loop {
            let state = (current_pos, current_facing);
            if visited_states.contains(&state) {
                // Found a loop!
                loop_count += 1;
                break;
            }
            visited_states.insert(state);

            if let Some(obstacle_pos) = test_map.find_facing(&current_pos, &MapElement::Obstacle, current_facing) {
                current_pos = obstacle_pos;
                current_facing = current_facing.rotate_cw();
            } else {
                // Guard exits the map, no loop
                break;
            }
        }
    }

    loop_count
}

fn print_map(map: &MapGrid, points: &[(Coordinate, Facing)]) {
    (0..map.height())
        .map(|y| {
            (0..map.width())
                .flat_map(move |x| {
                    if let Some(el) = map.get(&(x, y)) {
                        if let Some((_, face)) = points.iter().find(|(c, _)| c == &(x, y)) {
                            Some(match face {
                                Facing::Up => "^".to_string(),
                                Facing::Down => "v".to_string(),
                                Facing::Left => "<".to_string(),
                                Facing::Right => ">".to_string(),
                            })
                        } else {
                            Some(el.to_string())
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .for_each(|r| println!("{}", r));
}

#[cfg(test)]
mod test {
    use crate::{
        coordinate_abs_distance, coordinates_between, part_1, part_2, Facing, MapElement, MapGrid,
    };

    const INPUT_TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn can_replace_character_inside_map() {
        let mut map = MapGrid {
            elements: MapElement::try_parse(INPUT_TEST).expect("can parse input"),
        };

        map.set(&(0, 0), MapElement::Guard { face: Facing::Down });
        assert_eq!(
            map.get(&(0, 0)),
            Some(MapElement::Guard { face: Facing::Down })
        );
    }

    #[test]
    fn can_find_with_multiple_obstacle() {
        let map = MapGrid {
            elements: MapElement::try_parse(
                ".#..
..#.#
....#
.^...",
            )
            .expect("can parse input"),
        };

        assert_eq!(
            map.find_facing(&(0, 1), &MapElement::Obstacle, Facing::Right),
            Some((1, 1))
        );
    }

    #[test]
    fn can_find_coordinate_between() {
        let a = (10, 50);
        let b = (15, 50);

        assert_eq!(
            coordinates_between(a, b),
            vec![(10, 50), (11, 50), (12, 50), (13, 50), (14, 50), (15, 50)]
        );

        let a = (10, 50);
        let b = (10, 47);

        assert_eq!(
            coordinates_between(a, b),
            vec![(10, 47), (10, 48), (10, 49), (10, 50)]
        );
    }

    #[test]
    fn can_find_facing() {
        let map = MapGrid {
            elements: MapElement::try_parse(INPUT_TEST).expect("can parse input"),
        };

        assert_eq!(
            map.find_facing(&(4, 6), &MapElement::Obstacle, Facing::Up),
            Some((4, 1))
        );
    }

    #[test]
    fn can_find_abs_distance() {
        let a = (10, 50);
        let b = (10, 60);

        assert_eq!(coordinate_abs_distance(a, b), 10);

        let a = (1, 100);
        let b = (37, 100);

        assert_eq!(coordinate_abs_distance(a, b), 36);
    }

    #[test]
    fn can_find_anywhere() {
        let map = MapGrid {
            elements: MapElement::try_parse(INPUT_TEST).expect("can parse input"),
        };

        assert_eq!(
            map.find_anywhere(&MapElement::Guard { face: Facing::Up }),
            Some((4, 6))
        );
    }

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 41);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 6);
    }
}
