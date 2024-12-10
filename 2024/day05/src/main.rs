use std::{fs, str::FromStr};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("can read input");

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Copy, Clone, Debug)]
struct Rule {
    before: usize,
    after: usize,
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut content = s.split('|').take(2);
        let before = content.next();
        let after = content.next();

        if let (Some(before), Some(after)) = (before, after) {
            if let (Ok(before), Ok(after)) = (before.parse(), after.parse()) {
                Ok(Self { before, after })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl Rule {
    pub fn new(before: usize, after: usize) -> Self {
        Self { before, after }
    }

    /// Determines if the rules can be applied to the pages.
    /// Before and after must be contained in the pages
    pub fn can_apply(&self, pages: &[usize]) -> bool {
        pages.contains(&self.before) && pages.contains(&self.after)
    }

    pub fn is_valid(&self, pages: &[usize]) -> bool {
        let mut search = pages.iter();

        search
            .clone()
            .position(|p| *p == self.before)
            .expect("can get before position")
            < search
                .position(|p| *p == self.after)
                .expect("can get after position")
    }

    pub fn positions(&self, pages: &[usize]) -> Option<(usize, usize)> {
        let mut search = pages.iter();

        if let (Some(before), Some(after)) = (
            search.clone().position(|p| *p == self.before),
            search.position(|p| *p == self.after),
        ) {
            Some((before, after))
        } else {
            None
        }
    }

    pub fn apply(&self, pages: &[usize]) -> Option<Vec<usize>> {
        if let Some((a, b)) = self.positions(pages) {
            let mut pages = pages.to_owned();
            pages.swap(a, b);
            Some(pages)
        } else {
            None
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut parsed = input.split("\n\n");
    let mut count: usize = 0;
    let rules: Vec<Rule> = parsed
        .next()
        .expect("has rules")
        .split_whitespace()
        .map(|l| l.parse().expect("can parse rule"))
        .collect();

    for pages in parsed.next().expect("has pages").lines().map(|l| {
        l.split(',')
            .map(|p| p.parse().expect("can parse number"))
            .collect::<Vec<usize>>()
    }) {
        let rules = rules.clone();
        let mut valid_rules = rules.iter().filter(|r| r.can_apply(&pages));

        if valid_rules.all(|r| r.is_valid(&pages)) {
            count += pages.get(pages.len() / 2).expect("can split pages in half");
        }
    }

    count
}

fn part_2(input: &str) -> usize {
    let mut parsed = input.split("\n\n");
    let mut count: usize = 0;
    let rules: Vec<Rule> = parsed
        .next()
        .expect("has rules")
        .split_whitespace()
        .map(|l| l.parse().expect("can parse rule"))
        .collect();

    for pages in parsed.next().expect("has pages").lines().map(|l| {
        l.split(',')
            .map(|p| p.parse().expect("can parse number"))
            .collect::<Vec<usize>>()
    }) {
        let rules = rules.clone();
        let all_rules = rules.iter().filter(|r| r.can_apply(&pages));

        // seems like all consumes the iter, need to read more about that..
        if all_rules.clone().all(|r| r.is_valid(&pages)) {
            continue;
        }

        let mut pages = pages.to_owned();
        while all_rules.clone().any(|r| !r.is_valid(&pages)) {
            let first_invalid = all_rules
                .to_owned()
                .find(|r| !r.is_valid(&pages))
                .expect("can get first invalid rule");

            if let Some(new_pages) = first_invalid.apply(&pages) {
                pages = new_pages;
            }
        }

        count += pages
            .get(pages.len() / 2)
            .expect("can get half of the pages");
    }

    count
}

#[cfg(test)]
mod test {
    use crate::{part_1, part_2, Rule};

    const INPUT_TEST: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn rules_work_correctly() {
        let pages = [5, 10, 50, 30, 20];
        let r1 = Rule::new(10, 50);
        let r2 = Rule::new(5, 20);
        let r3 = Rule::new(30, 50);

        assert!(r1.is_valid(&pages));
        assert!(r2.is_valid(&pages));
        assert!(!r3.is_valid(&pages));

        let mut pages = [61, 13, 29];
        let r1 = Rule::new(29, 13);

        assert!(!r1.is_valid(&pages));
        assert_eq!(r1.positions(&pages), Some((2, 1)));
        pages.swap(2, 1);
        assert_eq!(r1.positions(&pages), Some((1, 2)));
    }

    #[test]
    fn solve_part_1() {
        assert_eq!(part_1(INPUT_TEST), 143);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(part_2(INPUT_TEST), 123);
    }
}
