use std::collections::{BinaryHeap, HashMap};

pub fn run(input: &mut [u128]) {
    let mut count_map = HashMap::new();
    for &num in input.iter() {
        *count_map.entry(num).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (&num, &count) in &count_map {
        heap.push((count, num));
    }

    if let Some(&(max_count, _)) = heap.peek() {
        if max_count > input.len().div_ceil(2) {
            input.to_vec().clear();
            return;
        }
    }

    let mut idx = 0;
    let mut current = heap.pop();
    let mut next = heap.pop();

    while let Some((mut count1, num1)) = current {
        while count1 > 0 {
            if let Some((mut count2, num2)) = next {
                input[idx] = num1;
                idx += 1;
                input[idx] = num2;
                idx += 1;

                count1 -= 1;
                count2 -= 1;

                if count2 == 0 {
                    next = heap.pop();
                } else {
                    next = Some((count2, num2));
                }
            } else {
                input[idx] = num1;
                idx += 1;
                count1 -= 1;
            }
        }
        current = next;
        next = heap.pop();
    }

    if let Some((_count, num)) = heap.pop() {
        input[idx] = num;
    }
}
