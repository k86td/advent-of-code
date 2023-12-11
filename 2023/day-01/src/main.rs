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

fn main() {
    let mut istr = INPUT.to_string();

    istr = istr.replace("eight", "8");
    istr = istr.replace("three", "3");
    istr = istr.replace("seven", "7");
    istr = istr.replace("four", "4");
    istr = istr.replace("five", "5");
    istr = istr.replace("nine", "9");
    istr = istr.replace("six", "6");
    istr = istr.replace("one", "1");
    istr = istr.replace("two", "2");

    dbg!(&istr);

    let splitted = istr.split('\n');
    let mut sum: Vec<usize> = Vec::default();
    for input in splitted.into_iter() {
        let ascii_symbols: Vec<char> = input.chars().filter(|c| c.is_ascii_digit()).collect();
        let fsymb = ascii_symbols.first();
        let lsymb = ascii_symbols.last();

        if let Some(f) = fsymb {
            if let Some(l) = lsymb {
                println!("{f}{l}");
                sum.push(format!("{}{}", f, l).parse().unwrap());
            }
        }
    }

    println!("sum: {}", sum.into_iter().sum::<usize>());
}
