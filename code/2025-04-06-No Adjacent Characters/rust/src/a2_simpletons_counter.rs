use std::collections::HashMap;

pub fn run(input: &[u128]) -> Vec<u128> {
    // println!("a2");

    if input.len() <= 1 {
        return input.to_vec();
    }

    let mut counts: HashMap<u128, usize> = HashMap::new();
    for &value in input {
        *counts.entry(value).or_default() += 1;
    }

    let mut elements: Vec<(u128, usize)> = counts.into_iter().collect();
    elements.sort_by(|a, b| b.1.cmp(&a.1)); 

    let max_count = elements[0].1;
    let n = input.len();
    if max_count > n.div_ceil(2) {
        // println!("a2. NOT POSSIBLE");
        return vec![];
    }

    let mut output = vec![0; n];
    let mut idx = 0;

    let flat: Vec<u128> = elements
        .iter()
        .flat_map(|(val, count)| std::iter::repeat_n(*val, *count))
        .collect();

    for val in flat {
        output[idx] = val;
        idx += 2;
        if idx >= n {
            idx = 1;
        }
    }

    output
}

