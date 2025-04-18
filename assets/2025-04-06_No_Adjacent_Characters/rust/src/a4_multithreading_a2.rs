use std::collections::HashMap;
use itertools::Itertools;
use rayon::prelude::*;
use rayon::iter::repeatn;

pub fn run(input: &[u128]) -> Vec<u128> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    // 1. Count frequencies in parallel
    let chunk_size = (input.len() / rayon::current_num_threads()).max(1);

    let counts = input
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut local = HashMap::new();
            for &val in chunk {
                *local.entry(val).or_default() += 1;
            }
            local
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (k, v) in map {
                *acc.entry(k).or_default() += v;
            }
            acc
        });

    // 2. Sort by frequency
    let mut elements: Vec<(u128, usize)> = counts.into_iter().collect();
    elements.sort_by(|a, b| b.1.cmp(&a.1));

    // 3. Validate that we can create a non-adjacent arrangement
    let max_count = elements[0].1;
    let n = input.len();
    if max_count > n.div_ceil(2) {
        return vec![];
    }

    // 4. Flatten into [a, a, a, b, b, c, ...]
    let flat: Vec<u128> = elements
        .into_par_iter()
        .flat_map(|(val, count)| repeatn(val, count))
        .collect();

        // 5. Fill result at even then odd indices in parallel
    let mut output = vec![0; n];

    // Split flat into even/odd slices
    let (even_vals, odd_vals): (Vec<_>, Vec<_>) = flat.into_iter()
        .enumerate()
        .partition(|&(i, _)| i < n.div_ceil(2));

    // Fill even indices
    output.par_chunks_mut(2)
        .zip(even_vals.into_par_iter().map(|(_, val)| val))
        .for_each(|(slot, val)| slot[0] = val);

    // Fill odd indices
    output.par_chunks_mut(2)
        .skip(1)
        .zip(odd_vals.into_par_iter().map(|(_, val)| val))
        .for_each(|(slot, val)| slot[0] = val);

    output
}

