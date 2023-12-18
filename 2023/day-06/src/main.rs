use std::fmt::Debug;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: usize,
    distance: usize,
}

impl PartialEq for Race {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.distance == other.distance
    }
}

impl Race {
    /// Returns a [`Vec`] for every winning time in
    /// this current [`Race`].
    fn get_wins(&self) -> Vec<usize> {
        (1..self.time)
            .filter(|t| (self.time - t) * t > self.distance)
            .collect()
    }
}

fn process_part_one(input: &str) -> usize {
    parse_input_multi(input)
        .iter()
        .map(|r| r.get_wins().len())
        .product()
}

fn process_part_two(input: &str) -> usize {
    parse_input_single(input).get_wins().len()
}

fn map_line_to_usize_multi(input: &str) -> impl Iterator<Item = usize> + '_ + Debug {
    input
        .split_whitespace()
        .map(|c| c.trim().parse::<usize>().unwrap())
}

fn map_line_to_usize_single(input: &str) -> usize {
    input.replace(' ', "").trim().parse().unwrap()
}

fn parse_input_single(input: &str) -> Race {
    let input = input.replace("Time:", "").replace("Distance:", "");
    let input: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let time = map_line_to_usize_single(input.first().unwrap());
    let distance = map_line_to_usize_single(input.last().unwrap());

    Race { time, distance }
}

fn parse_input_multi(input: &str) -> Vec<Race> {
    let input = input.replace("Time:", "").replace("Distance:", "");
    let input: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let time = map_line_to_usize_multi(input.first().unwrap());
    let distance = map_line_to_usize_multi(input.last().unwrap());

    time.zip(distance)
        .map(|r| Race {
            time: r.0,
            distance: r.1,
        })
        .collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let file_name = args.get(1).expect("can get file path");

    let part_1 = process_part_one(&std::fs::read_to_string(file_name).unwrap());
    let part_2 = process_part_two(&std::fs::read_to_string(file_name).unwrap());

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}
