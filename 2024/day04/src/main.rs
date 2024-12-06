fn main() {
    println!("Hello, world!");
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Describes a direction:
/// ```
/// TopLeft     Top   TopRight
/// Left              Right
/// BottomLeft Bottom BottomRight
/// ```
/// Where TopLeft is (-1, -1) and BottomRight
/// is (1, 1).
#[derive(Debug)]
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

impl Direction {
    pub fn to_matrix(&self) -> (isize, isize) {
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
}

struct SearchWindow<'a> {
    center: (usize, usize),
    grid: &'a Vec<Vec<char>>,
}

impl<'a> SearchWindow<'a> {
    pub fn new(center: (usize, usize), grid: &'a Vec<Vec<char>>) -> Self {
        Self { center, grid }
    }

    // checking ndarray, maybe it would help here?
    pub fn get(&self, dir: &Direction) -> Option<(&char, (usize, usize))> {
        let dir_coords = dir.to_matrix();
        let (x, y) = (
            self.center.0 as isize + dir_coords.0,
            self.center.1 as isize + dir_coords.1,
        );

        if let Some(row) = self.grid.get(y as usize).and_then(|l| l.get(x as usize)) {
            Some((row, (x as usize, y as usize)))
        } else {
            None
        }
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

    pub fn iter_neighbors(
        &self,
    ) -> impl Iterator<Item = (Option<(&char, (usize, usize))>, Direction)> {
        vec![
            Direction::TopLeft,
            Direction::Top,
            Direction::TopRight,
            Direction::Left,
            Direction::Right,
            Direction::BottomLeft,
            Direction::Bottom,
            Direction::BottomRight,
        ]
        .into_iter()
        .map(|d| (self.get(&d), d))
    }

    pub fn find(
        &self,
        target: char,
        constraint: Option<Direction>,
    ) -> Vec<((usize, usize), Direction)> {
        match constraint {
            Some(_) => todo!(),
            None => {
                let neighbors = self.iter_neighbors();
                neighbors
                    .filter(|(inner, _)| inner.is_some() && inner.unwrap().0 == &target)
                    .map(|(n, d)| (n.unwrap().1, d))
                    .collect()
            }
        }
    }
}

fn part_1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use crate::{parse, part_1, Direction, SearchWindow};

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
    fn searchwindow_finding_character() {
        let grid = parse(INPUT_TEST);
        let search_window = SearchWindow::new((0, 0), &grid);

        dbg!(search_window.find('M', None));

        panic!("log output");
    }

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 18);
    }
}
