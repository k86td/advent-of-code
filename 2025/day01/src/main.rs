use std::fs;

static TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

fn part_1(input: &str) -> usize {
    let mut count: isize = 0;
    let mut dial: isize = 50;
    input.lines().for_each(|l| {
        let ticks: isize = l[1..].parse().unwrap();
        if l.chars().nth(0) == Some('L') {
            dial = (dial - ticks).rem_euclid(100);
        } else {
            dial = (dial + ticks).rem_euclid(100);
        }

        // println!("Dial: {dial}");
        if dial == 0 {
            count += 1;
        }
    });

    count.try_into().unwrap()
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(TEST_INPUT), 3);
}

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();
    
    println!("Part 1: {}", part_1(&input));
}
