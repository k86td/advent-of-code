// day 01 of advent of code

const INPUT: &str = "
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

fn usize_from_token(input: &str) -> Option<char> {
    if input.contains("one") {
        Some('1')
    } else if input.contains("two") {
        Some('2')
    } else if input.contains("three") {
        Some('3')
    } else if input.contains("four") {
        Some('4')
    } else if input.contains("five") {
        Some('5')
    } else if input.contains("six") {
        Some('6')
    } else if input.contains("seven") {
        Some('7')
    } else if input.contains("eight") {
        Some('8')
    } else if input.contains("nine") {
        Some('9')
    } else {
        None
    }
}

fn get_first(input: &str) -> Option<char> {
    let mut curr_token = String::default();

    for ch in input.chars() {
        curr_token = format!("{}{}", &curr_token, ch);
        if let Some(t) = usize_from_token(&curr_token) {
            return Some(t);
        } else if ch.is_ascii_digit() {
            return Some(ch);
        }
    }

    None
}

fn get_last(input: &str) -> Option<char> {
    let mut curr_token = String::default();

    for ch in input.chars().rev() {
        curr_token = format!("{}{}", ch, &curr_token);
        if let Some(t) = usize_from_token(&curr_token) {
            return Some(t);
        } else if ch.is_ascii_digit() {
            return Some(ch);
        }
    }

    None
}

fn main() {
    let istr = INPUT.to_string();

    let splitted = istr.split('\n');
    let mut sum: Vec<usize> = Vec::default();
    for input in splitted.into_iter() {
        let fsymb = get_first(input);
        let lsymb = get_last(input);

        if let Some(f) = fsymb {
            if let Some(l) = lsymb {
                println!("{f}{l}");
                sum.push(format!("{}{}", f, l).parse().unwrap());
            }
        }
    }

    println!("sum: {}", sum.into_iter().sum::<usize>());
}

