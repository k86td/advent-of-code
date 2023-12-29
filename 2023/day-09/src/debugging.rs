// this file is for debugging purpose. Contains functions to do more visually appealing debugging and such...

use colored::Colorize;

use crate::{InternalSequence, Number};

impl ToString for InternalSequence {
    fn to_string(&self) -> String {
        let mut res = String::new();
        let biggest_len = self.sequences[0]
            .values
            .iter()
            .max()
            .unwrap()
            .to_string()
            .len();

        for (x, seq) in self.sequences.iter().enumerate() {
            let seq = seq
                .values
                .iter()
                .map(|v| format!("{:_<width$}", v.to_string(), width = biggest_len))
                .collect::<Vec<String>>()
                .join(" ");
            let rev_collapsed = self.rev_collapse_vec();
            let collapsed = self.collapse_vec();

            res += &format!(
                "{}{} {} {}\n",
                " ".repeat(x * biggest_len / 2 + x),
                rev_collapsed.get(x).unwrap().to_string().red(),
                seq.blue(),
                collapsed.get(x).unwrap().to_string().red()
            );
        }

        res
    }
}

impl InternalSequence {
    fn collapse_vec(&self) -> Vec<Number> {
        let mut res = Vec::new();

        let mut seq = self.sequences.clone();
        seq.reverse();
        seq.iter().fold(0, |mut acc, c| {
            acc += c.values.last().unwrap();
            res.push(acc.clone());
            acc
        });
        res.reverse();

        res
    }

    fn rev_collapse_vec(&self) -> Vec<Number> {
        let mut res = Vec::new();

        let mut seq = self.sequences.clone();
        seq.reverse();
        seq.iter().fold(0, |mut acc, c| {
            acc = c.values.first().unwrap() - acc;
            res.push(acc.clone());
            acc
        });
        res.reverse();

        res
    }
}
