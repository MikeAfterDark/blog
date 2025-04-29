use std::collections::HashMap;

pub fn run(input: &mut [u8]) {
    if input.len() <= 1 {
        return;
    }

    let mut counts: HashMap<u8, usize> = HashMap::new();
    for value in input.iter() {
        *counts.entry(*value).or_default() += 1;
    }
    
    let mut elements: Vec<(u8, usize)> = counts.into_iter().collect();
    elements.sort_by(|a, b| b.1.cmp(&a.1)); 

    let max_count = elements[0].1;
    let n = input.len();
    if max_count > n.div_ceil(2) {
        input.to_vec().clear();
        return;
    }

    let mut idx = 0;

    for (val, count) in elements {
        for _ in 0..count {
            input[idx] = val;
            idx += 2;
            if idx >= n {
                idx = 1;
            }
        }
    }
}

