use std::collections::HashMap;

pub fn factors_of(number: usize) -> Vec<usize> {
    let mut res = Vec::new();

    let mut current = number;
    while current % 2 == 0 {
        res.push(2);
        current /= 2;
    }

    for x in 3..=((current as f64).sqrt() + 1f64) as usize {
        if x % 2 == 0 {
            continue;
        }

        while current % x == 0 {
            res.push(x);
            current /= x;
        }
    }

    if current > 2 {
        res.push(current);
    }

    res
}

pub fn lcm_of(numbers: &[usize]) -> usize {
    let numbers_count = numbers
        .iter()
        .map(|n| {
            let mut res: HashMap<usize, usize> = HashMap::new();

            for factor in factors_of(*n) {
                res.entry(factor)
                    .and_modify(|n| {
                        *n += 1;
                    })
                    .or_insert(1);
            }

            res
        })
        .fold(HashMap::new(), |mut acc: HashMap<usize, usize>, hm| {
            for key in hm.keys() {
                let curr = hm.get(key).unwrap();
                acc.entry(*key)
                    .and_modify(|m| {
                        if *m < *curr {
                            *m = *curr;
                        }
                    })
                    .or_insert(*curr);
            }

            acc
        });

    let mut total = 1;
    for key in numbers_count.keys() {
        total *= key.pow(*numbers_count.get(key).unwrap() as u32);
    }

    total
}
