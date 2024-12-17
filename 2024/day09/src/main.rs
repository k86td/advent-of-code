use std::{
    collections::HashSet,
    fmt::{Display, Write},
    fs,
    num::ParseIntError,
    str::FromStr,
};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    File { id: usize },
    Empty,
}

impl BlockType {
    pub fn id(&self) -> Option<usize> {
        match self {
            BlockType::File { id } => Some(*id),
            BlockType::Empty => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StorageLayout {
    storage: Vec<BlockType>,
}

impl StorageLayout {
    pub fn is_fragmented(&self) -> bool {
        let mut free_started = false;

        for block in self.storage.iter() {
            match block {
                BlockType::File { id: _ } => {
                    if free_started {
                        return true;
                    }
                }
                BlockType::Empty => {
                    free_started = true;
                }
            }
        }

        false
    }
}

/// Parses disk map
impl FromStr for StorageLayout {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_iter = s.trim_end().chars();
        let mut id = 0;
        let mut layout = Vec::new();

        while let Some(next_char) = parse_iter.next() {
            // file
            let size = next_char.to_string().parse()?;
            layout.append(&mut (0..size).map(|_| BlockType::File { id }).collect());
            id += 1;

            // free space
            if let Some(next_char) = parse_iter.next() {
                let size: usize = next_char.to_string().parse()?;
                layout.append(&mut (0..size).map(|_| BlockType::Empty).collect());
            }
        }

        Ok(StorageLayout { storage: layout })
    }
}

impl Display for StorageLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.storage.iter().try_for_each(|b| match b {
            BlockType::File { id } => f.write_str(&id.to_string()),
            BlockType::Empty => f.write_char('.'),
        })
    }
}

fn part_1(input: &str) -> usize {
    let mut layout: StorageLayout = input.parse().expect("can get layout");

    while layout.is_fragmented() {
        let last_block_index = layout
            .storage
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, b)| {
                if matches!(b, BlockType::File { id: _ }) {
                    Some(i)
                } else {
                    None
                }
            })
            .expect("can get last block");

        let first_free_index = layout
            .storage
            .iter()
            .position(|b| matches!(b, BlockType::Empty))
            .expect("can get first free space");

        layout.storage.swap(last_block_index, first_free_index);
        layout.storage.remove(last_block_index);
    }

    layout
        .storage
        .into_iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let BlockType::File { id } = block {
                Some(index * id)
            } else {
                None
            }
        })
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut layout: StorageLayout = input.parse().expect("can get layout");

    let mut visited_ids: HashSet<Option<usize>> = HashSet::new();
    while layout.is_fragmented() {
        // println!("{}", layout);

        let mut first_id: Option<usize> = None;
        let last_block_index: Vec<usize> = layout
            .storage
            .iter()
            .enumerate()
            .rev()
            .skip_while(|(_, b)| matches!(b, BlockType::Empty) || visited_ids.contains(&b.id()))
            .take_while(|(_, b)| {
                if let Some(prev_id) = first_id {
                    match b {
                        BlockType::File { id } => prev_id == *id,
                        _ => false,
                    }
                } else {
                    first_id = b.id();
                    true
                }
            })
            .map(|(i, _)| i)
            .collect();

        if last_block_index.is_empty() {
            break;
        }

        visited_ids.insert(first_id);

        let free_spots = layout
            .storage
            .iter()
            .enumerate()
            .collect::<Vec<(usize, &BlockType)>>()
            .windows(last_block_index.len())
            .find(|e| e.iter().all(|(_, b)| matches!(b, BlockType::Empty)))
            .map(|e| e.iter().map(|(i, _)| *i).collect::<Vec<usize>>());

        if let Some(spots) = free_spots {
            for (file, spot) in spots.into_iter().zip(&last_block_index) {
                if *spot > file {
                    // println!(
                    //     "{}^{}^ ({},{})",
                    //     " ".repeat(file),
                    //     "-".repeat(spot - file - 1),
                    //     file,
                    //     spot
                    // );
                    layout.storage.swap(file, *spot);
                }
            }
        }
    }

    layout
        .storage
        .into_iter()
        .enumerate()
        .filter_map(|(index, block)| {
            if let BlockType::File { id } = block {
                Some(index * id)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2};

    const INPUT_TEST: &str = "2333133121414131402";

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 1928);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 2858);
    }
}
