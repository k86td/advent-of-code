const TEST_INPUT: &str = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

use crate::{process_part_one, process_part_two, Card, Game, Hand, HandState};

#[test]
fn card_compare() {
    assert!(Card::A > Card::K);
    assert!(Card::J > Card::Number(10));
    assert!(Card::Number(10) > Card::Number(2));
}

#[test]
fn getting_state() {
    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::A, Card::A, Card::A],
            bid: 1
        }
        .state(),
        HandState::FiveKind
    );

    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::A, Card::A],
            bid: 1
        }
        .state(),
        HandState::FourKind
    );

    assert_ne!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::A, Card::Number(9)],
            bid: 1
        }
        .state(),
        HandState::FullHouse
    );

    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::A, Card::Number(8)],
            bid: 1
        }
        .state(),
        HandState::FullHouse
    );

    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::A, Card::J],
            bid: 1
        }
        .state(),
        HandState::ThreeKind
    );

    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::J, Card::J],
            bid: 1
        }
        .state(),
        HandState::TwoPair
    );

    assert_eq!(
        Hand {
            cards: [Card::A, Card::A, Card::Number(8), Card::J, Card::K],
            bid: 1
        }
        .state(),
        HandState::OnePair
    );

    assert_eq!(
        Hand {
            cards: [
                Card::Number(2),
                Card::Number(3),
                Card::Number(8),
                Card::Number(4),
                Card::A
            ],
            bid: 1
        }
        .state(),
        HandState::HighCard
    );
}

#[test]
fn hand_bigger() {
    assert!(
        Hand {
            cards: [
                Card::Number(3),
                Card::Number(3),
                Card::Number(3),
                Card::Number(3),
                Card::Number(2)
            ],
            bid: 1
        } > Hand {
            cards: [Card::Number(2), Card::A, Card::A, Card::A, Card::A],
            bid: 1
        }
    );
}

#[test]
fn test_parse() {
    let input = "32T3K 765";

    assert_eq!(
        Game::from(input),
        Game {
            hands: vec![Hand {
                cards: [
                    Card::Number(3),
                    Card::Number(2),
                    Card::T,
                    Card::Number(3),
                    Card::K
                ],
                bid: 765
            }]
        }
    );
}

#[test]
fn part_1() {
    assert_eq!(process_part_one(TEST_INPUT), 6440);
}

#[test]
fn part_2() {
    assert_eq!(process_part_two(TEST_INPUT), 0);
}
