use std::fs;

#[derive(Debug, PartialEq)]
struct Robot {
    x: i32,
    y: i32,
    sx: i32,
    sy: i32,
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let mut nums = value.split(" ").flat_map(|a| {
            a.split("=")
                .skip(1)
                .take(1)
                .flat_map(|e| e.split(",").map(|n| n.parse().unwrap()))
        });

        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
            sx: nums.next().unwrap(),
            sy: nums.next().unwrap(),
        }
    }
}

impl Robot {
    fn pass(self, seconds: i32) -> Self {
        Robot {
            x: self.x + (self.sx * seconds),
            y: self.y + (self.sy * seconds),
            sx: self.sx,
            sy: self.sy,
        }
    }

    fn wrap(self, grid_size: (i32, i32)) -> Self {
        Robot {
            x: modulo(self.x, grid_size.0),
            y: modulo(self.y, grid_size.1),
            sx: self.sx,
            sy: self.sy,
        }
    }

    fn quadrant(self, grid_size: (i32, i32)) -> Quadrant {
        let mid_x = grid_size.0 / 2;
        let mid_y = grid_size.1 / 2;

        if self.x == mid_x || self.y == mid_y {
            return Quadrant::Middle;
        }

        match (self.x < mid_x, self.y < mid_y) {
            (true, true) => Quadrant::TopLeft,
            (true, false) => Quadrant::TopRight,
            (false, true) => Quadrant::BottomLeft,
            (false, false) => Quadrant::BottomRight,
        }
    }
}

#[derive(PartialEq)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Middle,
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input, (101, 103)));
    println!("part 2: {}", part_2(&input, (101, 103)));
}

fn part_1(input: &str, grid_size: (i32, i32)) -> usize {
    input
        .lines()
        .map(|r| Robot::from(r))
        .map(|r| r.pass(100).wrap(grid_size).quadrant(grid_size))
        .filter(|q| q != &Quadrant::Middle)
        .fold([0; 4], |mut quadrants, q| {
            quadrants[q as usize] += 1;
            quadrants
        })
        .iter()
        .product()
}

fn part_2(input: &str, grid_size: (i32, i32)) -> usize {
    0
}

fn modulo<T>(a: T, b: T) -> T
where
    T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy,
{
    ((a % b) + b) % b
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2, Robot};

    const INPUT_TEST: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST, (11, 7)), 12);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST, (11, 7)), 1);
    }

    #[test]
    fn can_parse_robot() {
        assert_eq!(
            Robot::from("p=39,70 v=-96,88"),
            Robot {
                x: 39,
                y: 70,
                sx: -96,
                sy: 88
            }
        )
    }

    #[test]
    fn can_pass_robot() {
        assert_eq!(
            Robot {
                x: 0,
                y: 0,
                sx: 2,
                sy: -45
            }
            .pass(4),
            Robot {
                x: 8,
                y: -180,
                sx: 2,
                sy: -45
            }
        )
    }

    #[test]
    fn can_wrap_robot() {
        assert_eq!(
            Robot {
                x: 2,
                y: 4,
                sx: 2,
                sy: -3
            }
            .pass(5)
            .wrap((11, 7)),
            Robot {
                x: 1,
                y: 3,
                sx: 2,
                sy: -3
            }
        )
    }
}
