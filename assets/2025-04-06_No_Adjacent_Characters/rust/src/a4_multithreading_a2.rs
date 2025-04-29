use std::collections::HashMap;
use rayon::prelude::*;
use rayon::iter::repeatn;

pub fn run(input: &mut [u8]) {
    if input.len() <= 1 {
        return;
    }

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

    let mut elements: Vec<(u8, usize)> = counts.into_iter().collect();
    elements.sort_by(|a, b| b.1.cmp(&a.1));

    let max_count = elements[0].1;
    let n = input.len();
    if max_count > n.div_ceil(2) {
        input.to_vec().clear();
        return;
    }

    let flat: Vec<u8> = elements
        .into_par_iter()
        .flat_map(|(val, count)| repeatn(val, count))
        .collect();

    let (even_vals, odd_vals): (Vec<_>, Vec<_>) = flat.into_iter()
        .enumerate()
        .partition(|&(i, _)| i < n.div_ceil(2));

    input.par_chunks_mut(2)
        .zip(even_vals.into_par_iter().map(|(_, val)| val))
        .for_each(|(slot, val)| slot[0] = val);

    input.par_chunks_mut(2)
        .skip(1)
        .zip(odd_vals.into_par_iter().map(|(_, val)| val))
        .for_each(|(slot, val)| slot[0] = val);
}

