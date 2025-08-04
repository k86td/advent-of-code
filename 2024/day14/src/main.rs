use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::{
    fs,
    io::{self, Write},
    time::Duration,
};

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
    fn pass(&self, seconds: i32) -> Self {
        Robot {
            x: self.x + (self.sx * seconds),
            y: self.y + (self.sy * seconds),
            sx: self.sx,
            sy: self.sy,
        }
    }

    fn revert(&self, seconds: i32) -> Self {
        Robot {
            x: self.x - (self.sx * seconds),
            y: self.y - (self.sy * seconds),
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

    fn visualize(robots: &[Robot], grid_size: (i32, i32)) {
        let mut grid = vec![vec![0; grid_size.0 as usize]; grid_size.1 as usize];

        for robot in robots {
            grid[robot.y as usize][robot.x as usize] += 1;
        }

        for row in grid {
            for count in row {
                if count == 0 {
                    print!(".");
                } else {
                    print!("{}", count);
                }
            }
            print!("\r\n");
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

/// Calculates the standard deviation of a collection of floating-point values.
/// 
/// This function computes the population standard deviation by:
/// 1. Calculating the mean of all values
/// 2. Computing the sum of squared differences from the mean
/// 3. Taking the square root of the variance
/// 
/// # Arguments
/// * `c` - A slice of f64 values
/// 
/// # Returns
/// The standard deviation as an f64
fn derivation(c: &[f64]) -> f64 {
    let mut mean = 0.0;
    for v in c {
        mean += v;
    }
    mean /= c.len() as f64;

    (c.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / c.len() as f64).sqrt()
}

/// Calculates the standard deviation of robot positions in both x and y dimensions.
/// 
/// This function extracts the x and y coordinates from all robots and computes
/// their respective standard deviations. This is useful for detecting when robots
/// cluster together (low standard deviation) which might indicate pattern formation.
/// 
/// # Arguments
/// * `robots` - A slice of Robot structs
/// 
/// # Returns
/// A tuple containing (x_std_dev, y_std_dev) where:
/// - x_std_dev: Standard deviation of x-coordinates
/// - y_std_dev: Standard deviation of y-coordinates
fn robot_derivation(robots: &[Robot]) -> (f64, f64) {
    let xs: Vec<f64> = robots.iter().map(|r| r.x as f64).collect();
    let ys: Vec<f64> = robots.iter().map(|r| r.y as f64).collect();

    (derivation(&xs), derivation(&ys))
}

// 10% decrease in std, something's up o.0
const STD_THRESHOLD: f64 = 0.9;

fn part_2(input: &str, grid_size: (i32, i32)) -> usize {
    let base_std_x = ((grid_size.0 as f64 - 1.0).powi(2) / 12.0).sqrt();
    let base_std_y = ((grid_size.1 as f64 - 1.0).powi(2) / 12.0).sqrt();

    let mut robots: Vec<Robot> = input
        .lines()
        .map(|r| Robot::from(r))
        // 8280
        // .map(|r| r.pass(8280).wrap(grid_size))
        .collect();

    let (x, y) = {
        let mut i = 1;
        let mut x_std: Option<usize> = None;
        let mut y_std: Option<usize> = None;
        loop {
            if x_std.is_some() && y_std.is_some() {
                break;
            }

            robots = robots.iter().map(|r| r.pass(1).wrap(grid_size)).collect();
            let (cx_std, cy_std) = robot_derivation(&robots);

            if cx_std < (base_std_x * STD_THRESHOLD) {
                x_std = Some(i);
            }

            if cy_std < (base_std_y * STD_THRESHOLD) {
                y_std = Some(i);
            }

            i += 1;
        }
        (
            x_std.expect("x_std is defined"),
            y_std.expect("y_std is defined"),
        )
    };

    dbg!(x, y);

    // Robot::visualize(&robots, grid_size);
    // let v = variation(&x);
    // dbg!(v, v.sqrt());

    // terminal::enable_raw_mode().unwrap();
    //
    // loop {
    //     print!("\x1B[2J\x1B[1;1H");
    //     Robot::visualize(&robots, grid_size);
    //
    //     let x: Vec<f64> = robots.iter().map(|r| r.x as f64).collect();
    //     let y: Vec<f64> = robots.iter().map(|r| r.y as f64).collect();
    //
    //     print!(
    //         "iter:{}, d_x:{}, d_y:{} (use arrow keys: right=forward, left=backward, q=quit)\r\n",
    //         &n,
    //         derivation(&x),
    //         derivation(&y),
    //     );
    //     io::stdout().flush().unwrap();
    //
    //     if event::poll(Duration::from_millis(100)).unwrap() {
    //         if let Event::Key(key_event) = event::read().unwrap() {
    //             match key_event.code {
    //                 KeyCode::Right => {
    //                     robots = robots
    //                         .into_iter()
    //                         .map(|r| r.pass(1).wrap(grid_size))
    //                         .collect();
    //                     n += 1;
    //                 }
    //                 KeyCode::Left => {
    //                     robots = robots
    //                         .into_iter()
    //                         .map(|r| r.revert(1).wrap(grid_size))
    //                         .collect();
    //                     n -= 1;
    //                 }
    //                 KeyCode::Char('q') => break,
    //                 _ => {}
    //             }
    //         }
    //     }
    // }
    //
    // terminal::disable_raw_mode().unwrap();

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
        // can't test this part:(
        assert_eq!(part_2(INPUT_TEST, (11, 7)), 0);
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
