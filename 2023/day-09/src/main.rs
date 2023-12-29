mod debugging;
#[cfg(test)]
mod test;

type Number = isize;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Sequence {
    values: Vec<Number>,
}

#[derive(PartialEq, Eq, Debug)]
struct InternalSequence {
    sequences: Vec<Sequence>,
}

impl From<&str> for Sequence {
    fn from(value: &str) -> Self {
        Sequence {
            values: value
                .split_whitespace()
                .map(|v| v.parse().unwrap())
                .collect(),
        }
    }
}

impl InternalSequence {
    fn collapse(&self) -> Number {
        self.sequences
            .iter()
            .fold(0, |acc, c| acc + c.values.last().unwrap())
    }

    fn rev_collapse(&self) -> Number {
        self.sequences
            .iter()
            .rev()
            .fold(0, |acc, c| c.values.first().unwrap() - acc)
    }
}

impl Sequence {
    fn internal_sequence(&self) -> Option<Sequence> {
        let cvalues = self.values.clone();
        let diff: Vec<Number> = cvalues
            .iter()
            .zip(cvalues.iter().skip(1))
            .map(|(curr, next)| next - curr)
            .collect();

        if diff.iter().sum::<Number>() == 0 {
            None
        } else {
            Some(Sequence { values: diff })
        }
    }

    fn get_internal_sequences(input: &str) -> InternalSequence {
        let mut curr: Option<Sequence> = Some(Sequence::from(input));
        let mut sequences = Vec::new();

        while let Some(internal) = curr {
            sequences.push(internal.clone());
            curr = internal.internal_sequence()
        }

        InternalSequence { sequences }
    }
}

fn parse(input: &str) -> Vec<InternalSequence> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| Sequence::get_internal_sequences(l))
        .collect()
}

fn process_part_one(input: &str) -> Number {
    parse(input).iter().map(|s| s.collapse()).sum()
}

fn process_part_two(input: &str) -> Number {
    parse(input).iter().map(|s| s.rev_collapse()).sum()
}

fn main() {
    let input = include_str!("../input");

    let part1 = process_part_one(&input);
    let part2 = process_part_two(&input);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
