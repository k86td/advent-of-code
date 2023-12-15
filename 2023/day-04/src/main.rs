mod test;

use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = fs::read_to_string(args.get(1).unwrap()).unwrap();

    println!("Part 1: {}", process_part_one(&input));
    println!("Part 2: {}", process_part_two(&input));
}

#[derive(Debug, Clone)]
pub struct Card {
    number: usize,
    winning: Vec<usize>,
    got: Vec<usize>,
}

pub fn process_line(input: &str) -> Card {
    let splitted = input.split(|s| s == ':' || s == '|');

    let mut winning: Vec<usize> = Vec::default();
    let mut got: Vec<usize> = Vec::default();
    let mut card_number: usize = 0;
    for (x, sec) in splitted.enumerate() {
        if x % 3 == 0 {
            // if it's "Game X"
            card_number = sec.replace("Card ", "").trim().parse().unwrap();
        } else if x % 3 == 1 {
            winning = sec
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        } else if x % 3 == 2 {
            // our numbers
            got = sec
                .split_whitespace()
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }
    }

    Card {
        number: card_number,
        winning,
        got,
    }
}

fn process_part_one(input: &str) -> usize {
    let mut sum: usize = 0;

    for line in input.lines() {
        let line = line.trim();
        if !(line == "") {
            let card = process_line(line);
            let mut win_count: usize = 0;
            for win in card.winning {
                if card.got.contains(&win) {
                    if win_count == 0 {
                        win_count = 1;
                    } else {
                        win_count *= 2;
                    }
                }
            }

            sum += win_count;
        }
    }

    sum
}

fn process_part_two(input: &str) -> usize {
    let mut sum: usize = 0;
    let mut cards: HashMap<usize, usize> = HashMap::default();

    for line in input.lines() {
        let line = line.trim();
        if !(line == "") {
            let card = process_line(line);
            let mut win_count: usize = 0;
            for win in card.winning {
                if card.got.contains(&win) {
                    win_count += 1;
                }
            }

            let current_active_cards = cards.entry(card.number).or_insert(1).clone();
            for i in card.number + 1..=card.number + win_count {
                let entry = cards.entry(i).or_insert(1);
                *entry += 1 * current_active_cards;
            }
        }
    }

    cards.values().sum()
}
