use core::panic;
use std::{
    borrow::{Borrow, BorrowMut},
    cmp::Ordering,
    collections::HashMap,
};

#[cfg(test)]
mod test;

#[derive(Debug, Hash, Clone, Copy)]
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
            Card::A => 13,
            Card::K => 12,
            Card::Q => 11,
            Card::T => 10,
            Card::J => 1,
        }
    }
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '0'..='9' => Card::Number(value.to_string().parse().unwrap()),
            _ => panic!("unknown symbol"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(match self.value().cmp(&other.value()) {
            o => o,
        })
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl Eq for Card {}

#[derive(Debug)]
enum HandState {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandState {
    fn value(&self) -> usize {
        match self {
            HandState::FiveKind => 7,
            HandState::FourKind => 6,
            HandState::FullHouse => 5,
            HandState::ThreeKind => 4,
            HandState::TwoPair => 3,
            HandState::OnePair => 2,
            HandState::HighCard => 1,
        }
    }
}

impl PartialEq for HandState {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
}

impl Hand {
    fn cmp_by_state(get_state: impl Fn(&Hand) -> HandState, a: &Hand, b: &Hand) -> Ordering {
        match get_state(a).value().cmp(&get_state(b).value()) {
            Ordering::Equal => {
                for c in a.cards.iter().zip(b.cards.iter()) {
                    match c.0.partial_cmp(c.1) {
                        Some(Ordering::Equal) => continue,
                        Some(o) => return o,
                        _ => (),
                    }
                }

                Ordering::Equal
            }
            o => o,
        }
    }

    fn count(&self) -> HashMap<Card, usize> {
        let mut count_map: HashMap<Card, usize> = HashMap::new();

        self.cards.iter().for_each(|card| {
            count_map.entry(*card).and_modify(|d| *d += 1).or_insert(1);
        });

        count_map
    }

    // possible count map:
    // fiveKind [5]
    // fourKind [4, 1]
    // fullHouse [3, 2]
    // threeKind [3, 1, 1]
    // twoPair [2, 2, 1]
    // onePair [2, 1, 1, 1]
    // highCard [1, 1, 1, 1, 1]
    fn state(&self) -> HandState {
        let counts = self.count();
        let mut values = counts.values().collect::<Vec<&usize>>();
        values.sort_by(|a, b| b.cmp(a));

        let mut values = values.iter();
        match values.next() {
            Some(5) => HandState::FiveKind,
            Some(4) => HandState::FourKind,
            Some(3) => match values.next() {
                Some(2) => HandState::FullHouse,
                Some(1) => HandState::ThreeKind,
                _ => panic!("impossible state using more than the available number of cards"),
            },
            Some(2) => match values.next() {
                Some(2) => HandState::TwoPair,
                Some(1) => HandState::OnePair,
                _ => panic!("impossible state using more than the available number of cards"),
            },
            _ => HandState::HighCard,
        }
    }

    fn replace_all(&mut self, target: Card, new: Card) {
        for index in 0..self.cards.len() {
            if self.cards[index] == target {
                self.cards[index] = new;
            }
        }
    }

    fn jokers_state(&self) -> HandState {
        let counts = self.count();
        let jokers_count = counts.get(&Card::J).unwrap_or(&0);

        if *jokers_count == 0 {
            return self.state();
        }

        let possibilities = counts.keys().filter(|c| match c {
            Card::J => false,
            _ => true,
        });

        let mut biggest_state = self.state();
        for poss in possibilities {
            let mut iter_hand = self.clone();
            iter_hand.replace_all(Card::J, *poss);

            if iter_hand.state().value() > biggest_state.value() {
                biggest_state = iter_hand.state();
            }
        }

        biggest_state
    }
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Hand::cmp_by_state(|hand| hand.state(), self, other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(other) {
            Some(o) => o,
            None => panic!("HandState should have a valid cmp"),
        }
    }
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

impl Eq for Game {}
impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.hands == other.hands
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let hands: Vec<Hand> = value
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| {
                let splitted: Vec<&str> = l.split(" ").collect();
                let unparsed_cards = splitted.get(0).unwrap().chars();
                let bid = splitted.get(1).unwrap().parse().unwrap();

                let mut cards = [Card::A; 5];
                for (index, ch) in unparsed_cards.enumerate() {
                    *cards.get_mut(index).unwrap() = Card::from(ch);
                }

                Hand { cards, bid }
            })
            .collect();

        Game { hands }
    }
}

impl Game {
    fn part_1(&self) -> usize {
        let mut hands = self.hands.clone();
        let mut bid = 0;
        hands.sort_by(|a, b| Hand::cmp_by_state(|h| h.state(), a, b));

        for (rank, hand) in hands.iter().enumerate() {
            // dbg!(&rank, &hand, &bid);
            bid += hand.bid * (rank + 1);
        }

        bid
    }

    fn part_2(&self) -> usize {
        let mut hands = self.hands.clone();
        let mut bid = 0;
        hands.sort_by(|a, b| Hand::cmp_by_state(|h| h.jokers_state(), a, b));

        for (rank, hand) in hands.iter().enumerate() {
            // dbg!(&rank, &hand, &bid);
            bid += hand.bid * (rank + 1);
        }

        bid
    }
}

fn process_part_one(input: &str) -> usize {
    Game::from(input).part_1()
}

fn process_part_two(input: &str) -> usize {
    Game::from(input).part_2()
}

fn main() {
    let input = include_str!("../input");

    let part1 = process_part_one(&input);
    let part2 = process_part_two(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
