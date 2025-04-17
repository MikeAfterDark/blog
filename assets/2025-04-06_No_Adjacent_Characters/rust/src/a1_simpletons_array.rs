pub fn run(input: &[u128]) -> Vec<u128> {
    // println!("a1");

    let mut vec = input.to_vec();
    let len = vec.len();

    if len < 2 {
        return vec;
    }

    let mut index = 0;
    let mut safeguard = len * 2;

    while !crate::utility::is_valid_vector(&vec) && safeguard > 0 {
        safeguard -= 1;

        if vec[index] == vec[index + 1] {
            let mut found_swap = false;

            for j in index + 2..len {
                if vec[j] != vec[index] {
                    vec.swap(index + 1, j);
                    found_swap = true;
                    break;
                }
            }

            if !found_swap {
                for j in 0..index {
                    if vec[j] != vec[index] {
                        vec.swap(index + 1, j);
                        found_swap = true;
                        break;
                    }
                }
            }

            if !found_swap {
                // println!("a1. Failed to fix conflict at index {}", index);
                return vec![];
            }
        }

        index = (index + 1) % (len - 1);
    }

    if safeguard == 0 {
        // println!("a1. Safeguard triggered: too many attempts to fix input.");
        return vec![];
    }

    vec
}

