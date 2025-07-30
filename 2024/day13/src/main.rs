use std::fs;

// using algebra, calculate using elimination method

#[derive(PartialEq, Debug)]
struct Button {
    x: f64,
    y: f64,
}

#[derive(PartialEq, Debug)]
struct Prize {
    tx: f64,
    ty: f64,
}

impl TryFrom<String> for Button {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let moves = value.split(":");
        let mut coord = moves.skip(1).flat_map(|e| {
            e.replace("X", "")
                .replace("Y", "")
                .split(",")
                .map(|i| i.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        });

        Ok(Button {
            x: coord.next().unwrap(),
            y: coord.next().unwrap(),
        })
    }
}

impl TryFrom<String> for Prize {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let prizes = value.split(":");
        let mut prize = prizes.skip(1).flat_map(|e| {
            e.replace("X=", "")
                .replace("Y=", "")
                .split(",")
                .map(|i| i.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()
        });

        Ok(Prize {
            tx: prize.next().unwrap(),
            ty: prize.next().unwrap(),
        })
    }
}

#[derive(PartialEq, Debug)]
struct Puzzle {
    ba: Button,
    bb: Button,
    prize: Prize,
}

impl TryFrom<String> for Puzzle {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut lines = value.trim().lines();

        Ok(Puzzle {
            ba: Button::try_from(lines.next().unwrap().to_string()).unwrap(),
            bb: Button::try_from(lines.next().unwrap().to_string()).unwrap(),
            prize: Prize::try_from(lines.next().unwrap().to_string()).unwrap(),
        })
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    let puzzles: Vec<Puzzle> = input
        .split("\n\n")
        .map(|p| Puzzle::try_from(p.to_string()).unwrap())
        .collect();

    for p in puzzles {
        let y = ((p.prize.tx * p.ba.y) - (p.prize.ty * p.ba.x))
            / ((p.ba.y * p.bb.x) - (p.bb.y * p.ba.x));

        if !(y.fract() == 0.0) {
            continue;
        }

        let x = (p.prize.tx - (p.bb.x * y)) / (p.ba.x);

        if !(x.fract() == 0.0) || (y >= 100.0 || x >= 100.0) {
            continue;
        }

        sum += y as usize + (x as usize * 3)
    }

    sum
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;
    let puzzles: Vec<Puzzle> = input
        .split("\n\n")
        .map(|p| {
            let mut p = Puzzle::try_from(p.to_string()).unwrap();
            p.prize.tx = p.prize.tx + 10000000000000.0;
            p.prize.ty = p.prize.ty + 10000000000000.0;
            p
        })
        .collect();

    for p in puzzles {
        dbg!(&p);
        let y = ((p.prize.tx * p.ba.y) - (p.prize.ty * p.ba.x))
            / ((p.ba.y * p.bb.x) - (p.bb.y * p.ba.x));

        if !(y.fract() == 0.0) {
            continue;
        }

        let x = (p.prize.tx - (p.bb.x * y)) / (p.ba.x);

        if !(x.fract() == 0.0) {
            continue;
        }

        sum += y as usize + (x as usize * 3)
    }

    sum
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 480);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 875318608908);
    }

    use crate::Button;

    #[test]
    fn test_from_for_button() {
        assert_eq!(
            Button::try_from("Button A: X+69, Y+23".to_string()),
            Ok(Button { x: 69.0, y: 23.0 })
        );
        assert_eq!(
            Button::try_from("Button B: X+230, Y+1".to_string()),
            Ok(Button { x: 230.0, y: 1.0 })
        );
    }

    use crate::Prize;

    #[test]
    fn test_from_for_prize() {
        assert_eq!(
            Prize::try_from("Prize: X=8400, Y=5400".to_string()),
            Ok(Prize {
                tx: 8400.0,
                ty: 5400.0
            })
        );
        assert_eq!(
            Prize::try_from("Prize: X=12748, Y=12176".to_string()),
            Ok(Prize {
                tx: 12748.0,
                ty: 12176.0
            })
        );
    }

    use crate::Puzzle;

    #[test]
    fn test_from_for_puzzle() {
        let puzzle_string = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
            .to_string();

        assert_eq!(
            Puzzle::try_from(puzzle_string),
            Ok(Puzzle {
                ba: Button { x: 94.0, y: 34.0 },
                bb: Button { x: 22.0, y: 67.0 },
                prize: Prize {
                    tx: 8400.0,
                    ty: 5400.0
                }
            })
        );
    }
}
