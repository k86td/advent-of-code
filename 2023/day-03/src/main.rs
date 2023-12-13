use std::{
    isize,
    ops::{Add, Mul, Sub},
};

mod test;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
struct Number {
    value: isize,
    begin: Coord,
    end: Coord,
}

#[derive(Debug, Copy, Clone)]
struct NumberBuilder {
    value: Option<isize>,
    begin: Option<Coord>,
    end: Option<Coord>,
}

impl NumberBuilder {
    pub fn build(&self) -> Number {
        Number {
            value: self.value.unwrap(),
            begin: self.begin.unwrap(),
            end: self.end.unwrap(),
        }
    }

    pub fn set_value(mut self, v: isize) -> NumberBuilder {
        self.value = Some(v);
        self
    }

    pub fn set_end(mut self, v: Coord) -> NumberBuilder {
        self.end = Some(v);
        self
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.begin == other.begin && self.end == other.end
    }
}

impl Number {
    pub fn is_in_range(&self, coord: Coord) -> bool {
        false
    }
}

fn parse_input(input: &str) -> (Vec<Coord>, Vec<Number>) {
    let mut symbols_loc = Vec::new();
    let mut numbers = Vec::new();

    let input: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    for (y, line) in input.iter().enumerate() {
        let y = y as isize;
        let mut curr_num: Option<NumberBuilder> = None;
        for (x, char) in line.chars().enumerate() {
            let x = x as isize;
            if char == '.' {
                if let Some(mut num) = curr_num {
                    num.end = Some(Coord(x - 1, y));
                    numbers.push(num.build());
                    curr_num = None;
                }

                // continue;
            } else if char.is_ascii_digit() {
                if let Some(num) = curr_num {
                    curr_num = Some(num.set_value(
                        num.value.unwrap() * 10 + char.to_string().parse::<isize>().unwrap(),
                    ));
                } else {
                    curr_num = Some(NumberBuilder {
                        value: Some(char.to_string().parse().unwrap()),
                        begin: Some(Coord(x, y)),
                        end: None,
                    });
                }
            } else {
                if let Some(num) = curr_num {
                    let num = num.set_end(Coord(x, y));
                    numbers.push(num.build());
                    curr_num = None;
                }

                symbols_loc.push(Coord(x, y));
            }

            if x == line.len() as isize - 1 {
                if let Some(num) = curr_num {
                    let num = num.set_end(Coord(x, y));
                    numbers.push(num.build());
                    curr_num = None;
                }
            }
        }
    }

    (symbols_loc, numbers)
}

#[derive(Debug, Clone, Copy)]
struct Coord(isize, isize);

impl Mul for Coord {
    type Output = Coord;

    fn mul(self, rhs: Self) -> Self::Output {
        Coord(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Coord {
    pub fn dot_product(&self, other: Coord) -> isize {
        let mut res = 0;
        res += self.0 * other.0;
        res += self.1 * other.1;

        res
    }
}

fn number_edge_of_symbol(symbol: Coord, number: Number) -> bool {
    for i in number.begin.0..=number.end.0 {
        let m = Coord(i, number.begin.1);
        let a = Coord(symbol.0 - 1, symbol.1 - 1);
        let b = Coord(symbol.0 + 1, symbol.1 - 1);
        let c = Coord(symbol.0 + 1, symbol.1 + 1);
        let d = Coord(symbol.0 + 1, symbol.1 + 1);

        let ab = b - a;
        let bc = c - b;
        let cd = d - c;
        let da = a - d;
        let am = m - a;
        let bm = m - b;
        let cm = m - c;
        let dm = m - d;

        let cond1 = 0 < am.dot_product(ab) && am.dot_product(ab) < ab.dot_product(ab);
        let cond2 = 0 < bm.dot_product(bc) && bm.dot_product(bc) < bc.dot_product(bc);
        let cond3 = 0 < cm.dot_product(cd) && cm.dot_product(cd) < cd.dot_product(cd);
        let cond4 = 0 < dm.dot_product(da) && dm.dot_product(da) < da.dot_product(da);

        if cond1 && cond2 && cond3 && cond4 {
            return true;
        }
    }

    false
}

pub fn process_part_one(input: &str) -> isize {
    let input = parse_input(input);

    0
}
