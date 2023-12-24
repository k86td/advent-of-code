use std::collections::HashMap;

#[cfg(test)]
mod test;

// RL
// AAA = (BBB, CCC)
// BBB = (DDD, EEE)
// CCC = (ZZZ, GGG)
// DDD = (DDD, DDD)
// EEE = (EEE, EEE)
// GGG = (GGG, GGG)
// ZZZ = (ZZZ, ZZZ)

// first -> AAA
// AAA.R (_, CCC)
// CCC.L (ZZZ, _)
// ZZZ -> last

#[derive(Hash, Debug, Clone, PartialEq)]
struct Node {
    left: String,
    right: String,
}

#[derive(Hash, Debug, PartialEq, Clone)]
enum Directions {
    Left,
    Right,
}

#[derive(PartialEq, Clone)]
struct Game {
    map: HashMap<String, Node>,
    direction: Vec<Directions>,
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        parse(value)
    }
}

impl std::fmt::Debug for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Game")
            .field("map (count)", &self.map.len())
            .field("direction (count)", &self.direction.len())
            .finish()
    }
}

impl Game {
    fn find_quickest(&self, target: &str) -> usize {
        let mut count = 0;
        let mut current = "AAA";
        'ex_loop: loop {
            for direction in self.direction.clone() {
                count += 1;
                match direction {
                    Directions::Left => current = &self.map.get(current).unwrap().left,
                    Directions::Right => current = &self.map.get(current).unwrap().right,
                }

                if current == target {
                    break 'ex_loop;
                }
            }
        }

        count
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let mut splitted = value[1..value.len() - 1].split(", ");

        Node {
            left: splitted.next().unwrap().to_string(),
            right: splitted.next().unwrap().to_string(),
        }
    }
}

fn parse_direction(input: &str) -> Vec<Directions> {
    input
        .chars()
        .map(|c| match c {
            'R' => Directions::Right,
            'L' => Directions::Left,
            _ => panic!("unknown directions"),
        })
        .collect()
}

fn parse(input: &str) -> Game {
    let mut map = HashMap::new();

    let mut lines = input.lines().filter(|l| !l.is_empty());

    let direction = parse_direction(lines.next().unwrap());

    for line in lines {
        let (name, node) = {
            let mut splitted = line.split(" = ");

            let name = splitted.next().unwrap();
            let node = Node::from(splitted.next().unwrap());

            (name.to_string(), node)
        };

        map.insert(name, node);
    }

    Game { map, direction }
}

fn process_part_one(input: &str) -> usize {
    let g = Game::from(input);
    g.find_quickest("ZZZ")
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
