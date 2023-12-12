use std::{collections::HashMap, thread::current};

fn main() {
    let input = "
Game 1: 9 red, 5 blue, 6 green; 6 red, 13 blue; 2 blue, 7 green, 5 red
Game 2: 6 red, 2 green, 2 blue; 12 green, 11 red, 17 blue; 2 blue, 10 red, 11 green; 13 green, 17 red; 15 blue, 20 red, 3 green; 3 blue, 11 red, 1 green
Game 3: 20 green, 1 blue, 7 red; 20 green, 7 blue; 18 red, 8 green, 3 blue; 7 red, 6 blue, 11 green; 11 red, 6 blue, 16 green
Game 4: 6 blue, 6 green; 2 blue, 5 green, 1 red; 9 blue, 1 red, 1 green; 1 red, 6 green, 8 blue; 4 green, 1 red, 1 blue
Game 5: 5 red, 4 blue, 11 green; 10 green, 3 blue, 18 red; 13 red, 13 green, 2 blue
Game 6: 1 blue, 15 green, 7 red; 2 blue, 4 green, 1 red; 1 blue, 13 green, 11 red; 2 blue, 10 red, 9 green
Game 7: 8 green, 1 blue, 1 red; 2 red, 2 green, 3 blue; 2 red, 1 blue
Game 8: 5 green, 9 blue, 2 red; 7 green, 3 red, 1 blue; 7 blue, 2 red, 1 green; 4 blue, 2 red, 14 green; 1 red, 5 blue, 12 green; 7 green, 9 blue, 3 red
Game 9: 11 red, 6 blue; 2 blue, 8 red, 9 green; 8 green, 13 red, 14 blue; 2 blue, 7 red, 9 green
Game 10: 3 red, 3 blue, 4 green; 4 red, 3 green, 2 blue; 13 red, 4 blue, 3 green; 6 blue, 5 green, 8 red; 10 red, 5 blue, 3 green
Game 11: 8 blue, 2 green, 4 red; 2 red, 13 blue, 2 green; 7 red, 3 green, 13 blue; 8 blue, 4 red; 12 blue, 6 red; 18 blue, 3 red, 1 green
Game 12: 15 red, 2 blue; 3 red, 5 green, 1 blue; 2 blue, 3 green, 6 red; 9 red, 4 green, 4 blue; 3 green
Game 13: 3 red, 3 green, 14 blue; 3 red, 14 blue, 1 green; 3 green, 4 blue; 7 blue, 1 red, 4 green
Game 14: 1 green, 2 red, 4 blue; 3 green, 5 blue, 11 red; 12 red, 2 green; 1 blue, 3 green, 4 red
Game 15: 1 red, 3 green, 4 blue; 2 red, 3 green, 2 blue; 10 green, 3 red, 3 blue; 5 red, 11 green, 3 blue
Game 16: 5 red, 12 blue, 12 green; 8 red, 5 blue; 11 green, 5 blue, 12 red; 4 green, 10 blue, 1 red; 1 blue
Game 17: 18 green, 15 red, 5 blue; 5 blue, 4 green, 14 red; 4 red, 7 blue, 9 green
Game 18: 2 red, 12 blue, 2 green; 15 blue, 4 red; 14 red; 11 red, 5 green, 5 blue
Game 19: 4 red, 2 blue, 4 green; 5 red; 7 green, 1 blue; 1 green, 4 red, 2 blue
Game 20: 5 green, 1 blue; 3 blue, 9 green; 14 blue, 7 green; 7 green, 1 red, 1 blue; 7 green, 2 blue
Game 21: 6 blue, 3 green, 8 red; 9 red, 1 green, 1 blue; 4 green, 7 red; 1 blue, 1 green, 12 red; 4 green, 9 red, 5 blue
Game 22: 1 red, 3 blue, 2 green; 12 green, 5 blue, 1 red; 1 green, 3 blue, 3 red; 1 red, 8 green, 2 blue
Game 23: 2 blue, 9 red, 14 green; 7 blue, 10 red; 7 blue, 7 green, 1 red
Game 24: 19 red, 3 green; 7 blue, 4 green, 12 red; 14 red, 3 green, 1 blue; 3 green, 14 red; 5 green, 7 blue, 18 red
Game 25: 12 red, 4 green, 3 blue; 3 blue, 12 red, 11 green; 3 red, 11 green, 2 blue
Game 26: 11 green, 2 red; 5 blue, 4 red; 1 green, 6 blue, 3 red; 9 red, 7 blue; 1 blue, 6 red, 1 green
Game 27: 10 red, 8 blue, 7 green; 6 green, 7 blue; 4 red, 10 green, 9 blue; 9 red, 2 green, 1 blue; 11 blue, 15 red, 9 green
Game 28: 3 blue, 2 red, 8 green; 3 red, 10 green; 11 green, 1 blue; 5 blue, 6 green, 7 red; 3 blue, 2 green
Game 29: 18 red, 1 blue; 3 red, 4 blue, 7 green; 1 blue, 16 green, 2 red; 3 blue, 6 green, 15 red; 1 red, 1 blue; 17 red, 6 green
Game 30: 10 red, 6 blue, 13 green; 2 green, 10 red, 4 blue; 4 red, 2 green, 2 blue
Game 31: 5 red, 13 green, 5 blue; 5 green, 12 blue, 5 red; 5 red, 3 green, 5 blue; 2 green, 3 red, 14 blue
Game 32: 2 blue, 14 red, 13 green; 11 red, 3 green, 1 blue; 9 red, 2 blue, 2 green; 5 blue, 3 red, 2 green; 4 blue, 8 green, 6 red; 12 red, 4 green, 5 blue
Game 33: 1 green, 15 blue; 1 red, 4 blue; 1 red, 1 green, 5 blue
Game 34: 1 green, 2 blue, 3 red; 11 red, 10 blue; 6 blue, 3 red
Game 35: 2 blue, 3 red, 1 green; 1 green, 9 blue, 8 red; 2 blue, 5 red; 2 green, 2 red, 2 blue; 1 red, 10 blue; 5 red, 9 blue
Game 36: 8 green, 1 red, 2 blue; 7 red, 5 green, 9 blue; 1 red, 10 green, 13 blue; 1 red, 10 green
Game 37: 1 green, 1 red; 2 green, 2 red; 2 green, 6 red; 7 red; 1 blue, 2 red
Game 38: 8 red, 7 green, 11 blue; 6 green, 10 blue, 11 red; 13 blue, 18 green, 7 red; 2 red, 7 blue, 12 green
Game 39: 4 blue, 8 red; 1 blue, 11 red, 2 green; 2 green, 3 blue, 12 red; 6 red, 1 green, 9 blue; 6 red, 1 blue, 1 green
Game 40: 2 blue, 17 red, 2 green; 4 red, 7 green; 4 blue, 1 green, 10 red; 6 green, 2 red; 6 red, 1 blue, 4 green; 5 green, 9 red, 4 blue
Game 41: 1 red, 8 blue; 3 green, 5 red, 3 blue; 8 blue, 1 green; 1 red, 9 blue; 5 red, 3 blue; 1 green, 4 red, 3 blue
Game 42: 7 green, 1 red, 10 blue; 11 blue, 1 green; 1 red, 17 blue, 2 green; 1 red, 4 green; 1 green, 3 blue; 11 blue, 1 red
Game 43: 5 green, 1 red; 5 blue, 3 green, 14 red; 7 green, 2 red, 11 blue; 3 red, 10 green, 4 blue; 5 green, 3 blue, 9 red; 8 green, 3 blue, 2 red
Game 44: 10 blue, 1 red, 2 green; 5 blue, 2 green, 2 red; 2 red, 2 green, 5 blue; 7 blue, 14 red, 1 green; 1 red, 2 green, 5 blue
Game 45: 16 green, 11 blue, 7 red; 6 blue, 8 red, 9 green; 7 green, 8 blue, 10 red; 13 red, 15 green, 8 blue; 3 red, 12 green
Game 46: 7 red, 2 green, 4 blue; 3 green, 7 blue; 2 blue, 5 red, 2 green; 3 green, 8 blue, 2 red
Game 47: 6 blue, 5 red; 5 red, 4 green, 5 blue; 4 green, 8 red; 5 red, 4 blue, 4 green; 5 blue, 5 green, 3 red; 5 blue, 2 green, 3 red
Game 48: 11 blue, 7 green, 2 red; 3 red, 8 green, 1 blue; 3 red
Game 49: 8 blue, 1 green, 3 red; 2 blue, 4 red; 6 red, 1 green; 2 red, 10 blue, 10 green
Game 50: 1 red, 8 green; 1 blue, 2 red, 8 green; 7 red, 1 blue; 7 red, 1 blue, 5 green; 6 green, 3 red
Game 51: 10 blue, 6 red; 10 red; 12 red, 5 blue; 11 red, 3 green, 3 blue
Game 52: 11 green, 7 red, 3 blue; 1 red, 9 blue, 8 green; 16 green, 2 blue, 8 red; 8 blue, 6 green; 3 blue, 5 red, 10 green; 8 red, 9 blue, 12 green
Game 53: 1 green, 4 blue, 11 red; 1 green, 12 red, 6 blue; 1 green, 5 red, 12 blue; 5 red, 11 blue; 1 blue, 11 red; 8 blue, 4 red, 1 green
Game 54: 3 blue, 2 green, 8 red; 2 blue, 5 red; 3 blue, 2 red, 2 green; 1 red, 9 blue; 5 red
Game 55: 1 green, 11 blue, 5 red; 16 blue, 11 green, 8 red; 16 blue, 2 red, 13 green
Game 56: 8 green, 6 blue, 6 red; 10 blue, 6 red, 9 green; 3 green, 13 blue, 6 red; 4 green, 5 blue, 3 red
Game 57: 6 green, 6 blue; 1 green, 1 red; 14 green, 1 blue
Game 58: 1 blue; 1 red; 1 red, 3 green, 1 blue; 1 red
Game 59: 5 green, 10 red; 1 green, 2 blue, 6 red; 8 red, 3 green, 2 blue; 4 green, 1 blue
Game 60: 2 red, 8 green; 1 blue, 3 green, 1 red; 2 green, 1 blue, 5 red; 1 red, 13 green, 1 blue; 4 red, 6 green, 1 blue
Game 61: 2 red, 2 green; 15 red, 1 green, 3 blue; 20 red; 7 red, 2 blue; 8 red, 5 blue, 1 green
Game 62: 4 green, 12 red, 14 blue; 11 red, 3 blue, 13 green; 6 green, 16 blue, 7 red; 7 red, 10 blue, 11 green
Game 63: 2 green, 8 red, 3 blue; 1 red; 2 blue, 8 red; 5 blue, 2 red; 1 green, 5 blue, 10 red; 1 green, 3 blue, 11 red
Game 64: 12 blue, 2 red, 4 green; 4 green, 3 red, 5 blue; 9 blue, 1 red, 4 green; 7 green, 7 blue, 1 red; 1 red, 10 blue, 2 green
Game 65: 4 blue, 2 green, 1 red; 1 blue, 4 red, 3 green; 5 green, 3 red; 1 red, 2 green, 15 blue; 3 blue, 3 red
Game 66: 1 red, 7 blue, 1 green; 3 red, 1 green, 1 blue; 1 green, 9 red, 2 blue; 2 green, 2 blue; 5 red, 3 green, 3 blue; 1 blue, 5 red
Game 67: 6 green; 17 green, 5 blue; 3 blue, 3 red, 9 green; 2 green, 4 blue; 1 red, 15 green
Game 68: 1 blue, 11 red, 8 green; 17 green, 3 blue, 8 red; 5 green, 8 red; 18 green, 7 red, 2 blue; 6 green
Game 69: 12 green, 13 blue, 2 red; 4 red, 14 green, 1 blue; 11 red, 15 green, 5 blue; 15 green, 9 red; 4 blue, 1 red, 5 green; 10 red, 20 green, 13 blue
Game 70: 6 red, 8 green, 7 blue; 5 blue, 1 red, 17 green; 2 red, 3 blue, 6 green; 7 blue, 1 red, 14 green; 7 red, 6 blue, 16 green
Game 71: 3 green, 3 blue, 3 red; 1 blue, 11 red, 2 green; 1 blue, 11 red
Game 72: 9 red, 17 blue, 1 green; 20 red, 3 green, 2 blue; 14 blue, 4 green, 11 red; 2 red, 12 blue, 7 green; 18 red, 13 blue, 7 green
Game 73: 6 green, 12 blue, 1 red; 10 blue, 5 red; 6 green, 17 blue, 3 red
Game 74: 1 green, 2 blue, 13 red; 2 blue, 2 green, 1 red; 2 green, 1 blue, 7 red; 1 red, 1 green
Game 75: 10 red, 2 green; 3 blue, 4 green; 9 red, 1 green
Game 76: 1 red, 3 green, 1 blue; 3 blue, 4 green, 6 red; 9 blue, 12 green, 2 red; 5 green, 1 red, 1 blue
Game 77: 3 blue, 4 red, 11 green; 8 green, 5 red; 7 blue, 11 green; 1 green, 3 blue, 6 red
Game 78: 15 blue, 5 green; 7 green, 9 blue; 7 green, 3 red, 2 blue
Game 79: 9 green, 6 red, 4 blue; 4 blue, 2 red, 14 green; 17 green, 2 blue, 4 red; 1 red, 2 green; 3 red, 3 green, 2 blue
Game 80: 1 green; 15 green, 1 red; 1 blue, 20 green, 1 red; 3 red, 15 green, 1 blue; 4 red, 3 green; 2 red, 18 green
Game 81: 4 blue, 1 green, 13 red; 13 blue, 19 red; 4 red, 13 blue; 8 blue, 10 red; 13 blue, 5 red; 1 green, 7 blue, 12 red
Game 82: 5 red, 3 blue; 4 red, 3 green, 9 blue; 19 blue, 1 green, 5 red; 5 green, 3 red, 10 blue
Game 83: 9 red, 3 blue, 5 green; 1 blue, 1 green, 11 red; 2 blue, 6 green, 18 red
Game 84: 2 green; 6 green, 5 red; 3 green, 1 red, 1 blue
Game 85: 2 blue, 6 red; 9 green, 5 red, 15 blue; 7 green, 10 red, 2 blue; 10 red, 6 blue, 2 green; 8 green, 5 red, 12 blue; 6 green, 5 blue, 6 red
Game 86: 2 blue, 12 red, 3 green; 3 red, 2 blue; 1 green, 2 blue, 2 red; 7 blue, 3 red, 1 green; 1 green, 2 blue, 5 red; 3 green, 14 red, 4 blue
Game 87: 3 blue, 1 green; 3 red, 2 blue, 1 green; 1 red, 3 blue; 10 red, 3 green; 5 red, 2 blue
Game 88: 3 blue, 9 red, 9 green; 9 blue, 11 red; 2 green, 11 blue; 2 blue, 14 red, 1 green; 7 green, 11 blue, 8 red; 9 red, 8 green, 3 blue
Game 89: 3 red, 1 blue, 16 green; 5 blue, 4 red, 3 green; 3 blue, 5 red, 5 green; 5 green, 8 blue, 2 red; 4 green, 2 red, 1 blue; 4 red, 1 green, 6 blue
Game 90: 7 green, 8 red; 1 blue, 7 green, 5 red; 4 green, 6 red
Game 91: 3 green, 6 red, 4 blue; 2 green, 9 red, 10 blue; 3 green, 12 blue; 1 red, 4 blue
Game 92: 12 green, 8 blue, 16 red; 6 red, 14 green, 4 blue; 3 green, 3 red, 10 blue; 9 blue, 6 red, 15 green; 14 green, 9 blue, 10 red
Game 93: 4 blue, 4 red, 9 green; 2 blue, 2 green, 6 red; 1 blue, 7 red; 7 blue, 17 red; 2 blue, 13 red, 10 green
Game 94: 4 green, 10 red; 9 red; 1 green, 3 blue, 14 red
Game 95: 9 green, 5 red; 3 blue, 11 red, 6 green; 4 red, 1 green; 13 green, 3 blue, 5 red; 1 red, 6 blue, 12 green; 7 red, 7 green
Game 96: 6 blue; 5 green, 2 blue, 2 red; 14 blue, 3 green
Game 97: 1 blue, 2 green, 5 red; 2 green, 8 blue, 9 red; 1 green, 8 blue, 6 red; 1 blue, 17 red; 2 green, 10 blue, 11 red
Game 98: 3 red, 12 blue, 2 green; 3 green, 4 blue, 4 red; 1 red, 11 blue, 2 green; 1 blue, 3 red
Game 99: 2 green, 9 red; 8 red, 4 green, 9 blue; 8 blue, 13 red; 10 green, 8 blue, 6 red; 11 green, 2 red, 13 blue
Game 100: 5 blue, 2 green, 7 red; 14 red, 15 green, 1 blue; 3 blue, 3 red; 8 green, 10 red, 6 blue; 6 blue, 4 red, 8 green
";

    println!("Result: {}", process_part_two(input));
}

fn get_block_count(input: &str) -> Option<(String, usize)> {
    let values: Vec<&str> = input.split(' ').collect();

    if values.len() == 2 {
        Some((values[1].to_string(), values[0].parse().unwrap()))
    } else {
        None
    }
}

/// Process a line and return HaspMap of
/// block colors and their count
fn process_line(blocks: &str) -> Vec<HashMap<String, usize>> {
    let mut colors: Vec<HashMap<String, usize>> = Vec::default();

    for subset in blocks.split(';') {
        let mut curr_map: HashMap<String, usize> = HashMap::default();
        for block in subset.trim().split(',') {
            if let Some((col_block, col_count)) = get_block_count(block.trim()) {
                if let Some(saved_col_count) = curr_map.get_mut(&col_block) {
                    *saved_col_count += col_count;
                } else {
                    curr_map.insert(col_block, col_count);
                }
            }
        }
        colors.push(curr_map.clone());
    }

    colors
}

/// Set filters for colors to ensure certain count
/// for every color
fn validate_color_map(
    input: Vec<HashMap<String, usize>>,
    filter: fn(HashMap<String, usize>) -> bool,
) -> bool {
    if input.len() > 1 {
        for col_input in input {
            if filter(col_input) {
                continue;
            } else {
                return false;
            }
        }

        true
    } else {
        false
    }
}

fn get_color_map(
    input: Vec<HashMap<String, usize>>,
    filter: fn(&mut HashMap<String, usize>),
) -> Option<HashMap<String, usize>> {
    if !input.is_empty() {
        let mut latest_res = HashMap::default();
        for mut col_input in input {
            filter(&mut col_input);
            latest_res = col_input.clone();
        }

        return Some(latest_res);
    }

    None
}

fn get_game_id(input: &str) -> usize {
    input
        .replace("Game ", "")
        .trim()
        .parse()
        .unwrap_or_default()
}

fn process_part_one(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        if line.trim() == "" {
            continue;
        }

        let line_splitted: Vec<&str> = line.split(':').collect();

        let game_id = get_game_id(line_splitted.first().unwrap());
        if validate_color_map(process_line(line_splitted.get(1).unwrap().trim()), |hm| {
            hm.get("red").unwrap_or(&0) <= &12
                && hm.get("green").unwrap_or(&0) <= &13
                && hm.get("blue").unwrap_or(&0) <= &14
        }) {
            sum += game_id;
        }
    }

    sum
}

fn process_part_two(input: &str) -> usize {
    let mut sum = 0;
    for line in input.split('\n') {
        if line.trim() == "" {
            continue;
        }

        // let line = line.replace(';', ",");
        let line_splitted: Vec<&str> = line.split(':').collect();

        let game_id = get_game_id(line_splitted.first().unwrap());

        let blocks = process_line(line_splitted.get(1).unwrap().trim());

        let mut lowest_required_block: HashMap<String, usize> = HashMap::new();
        lowest_required_block.insert("blue".to_string(), 0);
        lowest_required_block.insert("red".to_string(), 0);
        lowest_required_block.insert("green".to_string(), 0);

        for block in blocks {
            let green = block.get("green").unwrap_or(&0);
            let red = block.get("red").unwrap_or(&0);
            let blue = block.get("blue").unwrap_or(&0);

            let curr_green = lowest_required_block.get_mut("green").unwrap();
            if green > curr_green {
                *curr_green = *green;
            }

            let curr_blue = lowest_required_block.get_mut("blue").unwrap();
            if blue > curr_blue {
                *curr_blue = *blue;
            }

            let curr_red = lowest_required_block.get_mut("red").unwrap();
            if red > curr_red {
                *curr_red = *red;
            }
        }

        let red = lowest_required_block.get("red").unwrap();
        let blue = lowest_required_block.get("blue").unwrap();
        let green = lowest_required_block.get("green").unwrap();

        sum += red * blue * green;
    }

    sum
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::{get_block_count, get_game_id, process_line, process_part_one, process_part_two};

    #[test]
    pub fn part1() {
        let input: &str = "
        Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";

        assert_eq!(process_part_one(input), 8);
    }

    #[test]
    pub fn part2() {
        assert_eq!(
            process_part_two("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );

        assert_eq!(
            process_part_two(
                "
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
            "
            ),
            2286
        );
    }

    #[test]
    pub fn getting_cube_colors() {
        let mut f1: HashMap<String, usize> = HashMap::new();
        let mut f1_1: HashMap<String, usize> = HashMap::new();

        f1.insert("black".to_string(), 3);
        f1.insert("blue".to_string(), 4);
        f1_1.insert("red".to_string(), 1);

        assert_eq!(process_line("3 black, 4 blue; 1 red"), vec![f1, f1_1]);

        let mut f2: HashMap<String, usize> = HashMap::new();
        let mut f2_1: HashMap<String, usize> = HashMap::new();
        let mut f2_2: HashMap<String, usize> = HashMap::new();

        f2.insert("black".to_string(), 3);
        f2.insert("blue".to_string(), 4);
        f2_1.insert("red".to_string(), 1);
        f2_2.insert("red".to_string(), 2);

        assert_eq!(
            process_line("3 black, 4 blue; 1 red; 2 red"),
            vec![f2, f2_1, f2_2]
        );
    }

    #[test]
    pub fn getting_cube_number_and_color() {
        assert_eq!(get_block_count("3 red"), Some(("red".to_string(), 3)));
        assert_eq!(get_block_count("14 red"), Some(("red".to_string(), 14)));
        assert_eq!(get_block_count("1 blue"), Some(("blue".to_string(), 1)));
        assert_eq!(get_block_count("78 green"), Some(("green".to_string(), 78)));
    }

    #[test]
    pub fn getting_game_id() {
        assert_eq!(get_game_id("Game 15"), 15);
        assert_eq!(get_game_id("Game 1"), 1);
        assert_eq!(get_game_id("Game 99"), 99);
        assert_eq!(get_game_id("Game 100"), 100);
        assert_eq!(get_game_id("Game 1058"), 1058);
    }
}
