pub fn run(input: &mut [u8]) {
    let len = input.len();

    if len < 2 {
        return;
    }

    let mut index = 0;
    let mut safeguard = len * 2;

    while !crate::utility::is_valid_vector(input) && safeguard > 0 {
        safeguard -= 1;

        if input[index] == input[index + 1] {
            let mut found_swap = false;

            for j in index + 2..len {
                if input[j] != input[index] {
                    input.swap(index + 1, j);
                    found_swap = true;
                    break;
                }
            }

            if !found_swap {
                for j in 0..index {
                    if input[j] != input[index] {
                        input.swap(index + 1, j);
                        found_swap = true;
                        break;
                    }
                }
            }

            if !found_swap {
                input.to_vec().clear();
                return;
            }
        }

        index = (index + 1) % (len - 1);
    }

    if safeguard == 0 {
        input.to_vec().clear();
    }
}

