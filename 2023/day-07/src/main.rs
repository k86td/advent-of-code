use std::cmp::Ordering;

#[cfg(test)]
mod test;

#[derive(Debug)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Number(usize),
}

impl Card {
    fn value(&self) -> usize {
        match self {
            Card::Number(n) => *n,
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.value().cmp(&other.value()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Greater,
        })
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

enum HandState {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand {
    cards: [Card; 5],
    bid: usize,
}

fn process_part_one(input: &str) -> usize {
    0
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

