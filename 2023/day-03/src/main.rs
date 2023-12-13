mod test;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Copy, Clone)]
struct Number {
    value: isize,
    begin: (isize, isize),
    end: (isize, isize),
}

#[derive(Debug, Copy, Clone)]
struct NumberBuilder {
    value: Option<isize>,
    begin: Option<(isize, isize)>,
    end: Option<(isize, isize)>,
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

    pub fn set_end(mut self, v: (isize, isize)) -> NumberBuilder {
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
    pub fn is_in_range(&self, coord: (isize, isize)) -> bool {
        false
    }
}

fn parse_input(input: &str) -> (Vec<(isize, isize)>, Vec<Number>) {
    let mut symbols_loc = Vec::new();
    let mut numbers = Vec::new();

    let input: Vec<&str> = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();

    for (y, line) in input.iter().enumerate() {
        let mut curr_num: Option<NumberBuilder> = None;
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                if let Some(mut num) = curr_num {
                    num.end = Some((x - 1, y));
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
                        begin: Some((x, y)),
                        end: None,
                    });
                }
            } else {
                if let Some(num) = curr_num {
                    let num = num.set_end((x, y));
                    numbers.push(num.build());
                    curr_num = None;
                }

                symbols_loc.push((x, y));
            }

            if x == line.len() - 1 {
                if let Some(num) = curr_num {
                    let num = num.set_end((x, y));
                    numbers.push(num.build());
                    curr_num = None;
                }
            }
        }
    }

    (symbols_loc, numbers)
}

fn check_number_with_symbol(symbol: (isize, isize), number: Number) -> bool {
    false
}

pub fn process_part_one(input: &str) -> isize {
    let input = parse_input(input);

    0
}
