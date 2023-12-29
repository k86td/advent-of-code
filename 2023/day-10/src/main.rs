use std::collections::HashSet;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Pipe {
    TopDown,
    LeftRight,
    TopRight,
    TopLeft,
    DownLeft,
    DownRight,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Top,
    Right,
    Down,
    Left,
}

impl Pipe {
    fn get_allowed_directions(&self) -> Vec<Direction> {
        match self {
            Pipe::TopDown => vec![Direction::Top, Direction::Down],
            Pipe::LeftRight => vec![Direction::Left, Direction::Right],
            Pipe::TopRight => vec![Direction::Top, Direction::Right],
            Pipe::TopLeft => vec![Direction::Top, Direction::Left],
            Pipe::DownLeft => vec![Direction::Down, Direction::Left],
            Pipe::DownRight => vec![Direction::Down, Direction::Right],
            Pipe::Ground => Vec::new(),
            Pipe::Start => vec![
                Direction::Top,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    coord: (usize, usize),
    pipe: Pipe,
}

impl Pos {
    fn is_connected(&self, other: &Pos) -> bool {
        let (x, y) = self.coord;

        if y > 0 && (x, y - 1) == other.coord // pipe on top
            && self.pipe.get_allowed_directions().contains(&Direction::Top)
            && other
                .pipe
                .get_allowed_directions()
                .contains(&Direction::Down)
        {
            true
        } else if (x + 1, y) == other.coord // pipe on right
            && self.pipe.get_allowed_directions().contains(&Direction::Right)
            && other
                .pipe
                .get_allowed_directions()
                .contains(&Direction::Left)
        {
            true
        } else if (x, y + 1) == other.coord // pipe down
            && self.pipe.get_allowed_directions().contains(&Direction::Down)
            && other
                .pipe
                .get_allowed_directions()
                .contains(&Direction::Top)
        {
            true
        } else if x > 0 && (x - 1, y) == other.coord // pipe on left
            && self.pipe.get_allowed_directions().contains(&Direction::Left)
            && other
                .pipe
                .get_allowed_directions()
                .contains(&Direction::Right)
        {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Map {
    start_loc: Pos,
    pos: Vec<Vec<Pos>>,
}

impl Map {
    fn left(&self, pos: &Pos) -> Option<&Pos> {
        if pos.coord.0 == 0 {
            None
        } else if let Some(p) = self.pos.get(pos.coord.1).unwrap().get(pos.coord.0 - 1) {
            Some(p)
        } else {
            None
        }
    }

    fn right(&self, pos: &Pos) -> Option<&Pos> {
        if let Some(p) = self.pos.get(pos.coord.1).unwrap().get(pos.coord.0 + 1) {
            Some(p)
        } else {
            None
        }
    }

    fn top(&self, pos: &Pos) -> Option<&Pos> {
        if pos.coord.1 == 0 {
            None
        } else if let Some(p) = self.pos.get(pos.coord.1 - 1) {
            Some(p.get(pos.coord.0).unwrap())
        } else {
            None
        }
    }

    fn down(&self, pos: &Pos) -> Option<&Pos> {
        if let Some(p) = self.pos.get(pos.coord.1 + 1) {
            Some(p.get(pos.coord.0).unwrap())
        } else {
            None
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut pos: Vec<Vec<Pos>> = Vec::new();
        let mut start_loc = None;
        for (y, line) in value.lines().filter(|l| !l.is_empty()).enumerate() {
            let mut line_vec = Vec::new();
            for (x, ch) in line.chars().enumerate() {
                let char_pipe = match ch {
                    '|' => Pipe::TopDown,
                    '-' => Pipe::LeftRight,
                    'L' => Pipe::TopRight,
                    'J' => Pipe::TopLeft,
                    '7' => Pipe::DownLeft,
                    'F' => Pipe::DownRight,
                    '.' => Pipe::Ground,
                    'S' => {
                        start_loc = Some(Pos {
                            pipe: Pipe::Start,
                            coord: (x, y),
                        });
                        Pipe::Start
                    }
                    _ => panic!("unknown character!"),
                };

                line_vec.push(Pos {
                    pipe: char_pipe,
                    coord: (x, y),
                });
            }

            pos.push(line_vec);
        }

        Map {
            start_loc: start_loc.unwrap(),
            pos,
        }
    }
}

fn get_connected(map: &Map, pos: &Pos) -> [Pos; 2] {
    let mut res: Vec<Pos> = Vec::new();

    if let Some(p) = map.left(pos) {
        if pos.is_connected(p) {
            res.push(*p);
        }
    }
    if let Some(p) = map.right(pos) {
        if pos.is_connected(p) {
            res.push(*p);
        }
    }
    if let Some(p) = map.top(pos) {
        if pos.is_connected(p) {
            res.push(*p);
        }
    }
    if let Some(p) = map.down(pos) {
        if pos.is_connected(p) {
            res.push(*p);
        }
    }

    [res[0], res[1]]
}

fn process_part_one(input: &str) -> usize {
    let map = Map::from(input);

    let mut index = 0;
    let mut last = map.start_loc;
    let mut found: Vec<Pos> = vec![map.start_loc];
    loop {
        index += 1;

        let last_connected = get_connected(&map, &last);
        let elem = last_connected.iter().filter(|p| !found.contains(p)).nth(0);

        if let Some(p) = elem {
            found.push(*p);
            last = *p;
        } else {
            break;
        }
    }

    index / 2
}

fn process_part_two(input: &str) -> usize {
    0
}

fn main() {
    let input = include_str!("../input");

    let part1 = process_part_one(&input);
    let part2 = process_part_two(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
