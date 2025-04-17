use std::collections::{BinaryHeap, HashMap};

pub fn run(input: &[u128]) -> Vec<u128> {
    let mut count_map = HashMap::new();
    for &num in input {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (&num, &count) in &count_map {
        heap.push((count, num));
    }

    if let Some(&(max_count, _)) = heap.peek() {
        if max_count > input.len().div_ceil(2) {
            return vec![];
        }
    }

    let mut result = vec![0; input.len()];
    while heap.len() >= 2 {
        let (count1, num1) = heap.pop().unwrap();
        let (count2, num2) = heap.pop().unwrap();

        result.push(num1);
        result.push(num2);

        if count1 > 1 {
            heap.push((count1 - 1, num1));
        }
        if count2 > 1 {
            heap.push((count2 - 1, num2));
        }
    }

    if let Some((_count, num)) = heap.pop() {
        result.push(num);
    }

    result
}
