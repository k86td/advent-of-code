use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

struct ElementPosition<'a> {
    element: &'a char,
    position: (usize, usize),
}

/// Describes a direction:
/// ```
/// TopLeft     Top   TopRight
/// Left              Right
/// BottomLeft Bottom BottomRight
/// ```
/// Where TopLeft is (-1, -1) and BottomRight
/// is (1, 1).
#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    TopLeft,
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

#[derive(Debug, Clone, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

#[derive(Debug, Clone, Copy)]
enum Constraint {
    Directional(Direction),
    Oriental(Orientation),
    None,
}

impl Constraint {
    pub fn to_vec(self) -> Vec<Direction> {
        match self {
            Constraint::Directional(d) => vec![d],
            Constraint::Oriental(o) => match o {
                Orientation::Horizontal => vec![Direction::Left, Direction::Right],
                Orientation::Vertical => vec![Direction::Top, Direction::Bottom],
                Orientation::Diagonal => vec![
                    Direction::TopLeft,
                    Direction::TopRight,
                    Direction::BottomLeft,
                    Direction::BottomRight,
                ],
            },
            Constraint::None => vec![
                Direction::TopLeft,
                Direction::Top,
                Direction::TopRight,
                Direction::Left,
                Direction::Right,
                Direction::BottomLeft,
                Direction::Bottom,
                Direction::BottomRight,
            ],
        }
    }
}

impl Direction {
    pub fn to_matrix(self) -> (isize, isize) {
        match self {
            Direction::TopLeft => (-1, -1),
            Direction::Top => (0, -1),
            Direction::TopRight => (1, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::BottomLeft => (-1, 1),
            Direction::Bottom => (0, 1),
            Direction::BottomRight => (1, 1),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::TopLeft => Direction::BottomRight,
            Direction::Top => Direction::Bottom,
            Direction::TopRight => Direction::BottomLeft,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::BottomLeft => Direction::TopRight,
            Direction::Bottom => Direction::Top,
            Direction::BottomRight => Direction::TopLeft,
        }
    }
}

#[derive(Clone)]
struct SearchWindow<'a> {
    center: (usize, usize),
    grid: &'a Vec<Vec<char>>,
}

impl<'a> SearchWindow<'a> {
    pub fn new(center: (usize, usize), grid: &'a Vec<Vec<char>>) -> Self {
        Self { center, grid }
    }

    pub fn slide_mut(&mut self, dir: &Direction) {
        if let Some(coords) = self.apply_matrix(dir) {
            self.center = coords;
        }
    }

    // checking ndarray, maybe it would help here?
    pub fn get(&self, dir: &Direction) -> Option<ElementPosition> {
        let dir_coords = dir.to_matrix();
        let (x, y) = (
            self.center.0 as isize + dir_coords.0,
            self.center.1 as isize + dir_coords.1,
        );

        self.grid
            .get(y as usize)
            .and_then(|l| l.get(x as usize))
            .map(|row| ElementPosition {
                element: row,
                position: (x as usize, y as usize),
            })
    }

    fn apply_matrix(&self, dir: &Direction) -> Option<(usize, usize)> {
        let dir_coords = dir.to_matrix();
        let (x, y) = (
            self.center.0 as isize + dir_coords.0,
            self.center.1 as isize + dir_coords.1,
        );

        if x < 0 || y < 0 {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    fn iter_neighbors(
        &self,
        valid_directions: Vec<Direction>,
    ) -> impl Iterator<Item = (Option<ElementPosition>, Direction)> {
        valid_directions.into_iter().map(|d| (self.get(&d), d))
    }

    pub fn find(&self, target: char, constraint: Constraint) -> Vec<((usize, usize), Direction)> {
        let neighbors = self.iter_neighbors(constraint.to_vec());

        neighbors
            .filter(|(inner, d)| {
                inner.is_some()
                    && inner.as_ref().unwrap().element == &target
                    && constraint.to_vec().contains(d)
            })
            .map(|(n, d)| (n.unwrap().position, d))
            .collect()
    }
}

// this is probably brain dead and could be done much more elegantly using filters and other fancy
// things
fn part_1(input: &str) -> usize {
    let grid = parse(input);
    let mut count: usize = 0;
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if *grid.get(y).unwrap().get(x).unwrap() == 'X' {
                let search = SearchWindow::new((x, y), &grid);
                for potential_search in search.find('M', Constraint::None) {
                    let mut result: Vec<(usize, usize)> = Vec::new();
                    let mut directional_search = search.clone();

                    directional_search.slide_mut(&potential_search.1);
                    let char_a =
                        directional_search.find('A', Constraint::Directional(potential_search.1));
                    if !char_a.is_empty() {
                        let char_a = &char_a.first().unwrap();
                        directional_search.slide_mut(&char_a.1);
                        let char_s =
                            directional_search.find('S', Constraint::Directional(char_a.1));
                        if !char_s.is_empty() {
                            let char_s = char_s.first().unwrap();

                            result.push((x, y));
                            result.push(potential_search.0);
                            result.push(char_a.0);
                            result.push(char_s.0);
                        }
                    }

                    if result.len() == 4 {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let grid = parse(input);
    let mut count: usize = 0;
    for y in 0..grid.len() {
        for x in 0..grid.first().unwrap().len() {
            if *grid.get(y).unwrap().get(x).unwrap() == 'A' {
                let search = SearchWindow::new((x, y), &grid);

                let found_m = search.find('M', Constraint::Oriental(Orientation::Diagonal));
                if found_m.len() != 2 {
                    continue;
                }

                let found_s: Vec<((usize, usize), Direction)> = found_m
                    .iter()
                    .map(|m| {
                        search
                            .clone()
                            .find('S', Constraint::Directional(m.1.opposite()))
                    })
                    .filter(|s| !s.is_empty())
                    .map(|s| *s.first().unwrap())
                    .collect();

                if found_s.len() == 2 {
                    count += 1;
                }
            }
        }
    }

    count
}

#[cfg(test)]
mod test {
    use crate::{parse, part_1, part_2, SearchWindow};

    const INPUT_TEST: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 18);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 9);
    }
}
