#!/bin/bash

data_output_folder="./results/data"
charts_output_folder="./results/charts"
program_path="./target/release/rust"

cargo build --release

iterations=7

# Functions:
generate_data() {
    local algo=$1
    local max_length=$2
    local chars=$3

    output_file="${data_output_folder}/${algo}_${max_length}_${chars}.csv"
    $program_path --algo "$algo" "$max_length" "$chars" --iterations "$iterations" --increment --output "$output_file"
}

generate_chart() {
    local char_count=$1
    local filename=$2
    shift 2

    local data_filenames=""

    while [[ $# -gt 1 ]]; do
        local algo=$1
        local length=$2
        shift 2

        data_filenames+=" ${data_output_folder}/${algo}_${length}_${char_count}.csv"
    done

    python3 ./plot_csv.py string_length success_avg_ns fail_avg_ns \
        "${char_count} chars" \
        "${charts_output_folder}/${filename}" \
        $data_filenames
}

# //=== Data Generation ===\\
# Syntax:
# generate_data algoIndex maxLength chars

# echo "Start time: $(date)"
#
# generate_data 1 1000 2
# generate_data 1 1200 3
# generate_data 1 2000 255
#
# echo "finished all 1's: $(date)"
#
# generate_data 2 1000 2
# generate_data 2 1200 3
# generate_data 2 2000 255
#
# generate_data 2 150000 2
# generate_data 2 150000 3
# generate_data 2 150000 500
# generate_data 2 150000 10000
#
# echo "finished all 2's: $(date)"
#
# generate_data 3 150000 2
# generate_data 3 150000 3
# generate_data 3 150000 500
# generate_data 3 150000 10000
#
# echo "finished all 3's: $(date)"
#
# generate_data 4 150000 2
# generate_data 4 150000 3
# generate_data 4 150000 500
# generate_data 4 150000 10000
#
# echo "finished all 4's: $(date)"
#
# generate_data 5 150000 2
# generate_data 5 150000 3
# generate_data 5 150000 500
# generate_data 5 150000 10000
#
# echo "End time: $(date)"

# //=== Chart Generation: ===\\
# Syntax:
# generate_chart #chars filename algoIndex length algoIndex length... etc

# generate_chart 2 comparing_algos_1_2_with_2_chars 1 1000 2 1000
# generate_chart 3 comparing_algos_1_2_with_3_chars 1 1200 2 1200
# generate_chart 255 comparing_algos_1_2_with_255_chars 1 2000 2 2000

# generate_chart 2 comparing_algos_2_3_with_2_chars 2 20000 3 20000
# generate_chart 3 comparing_algos_2_3_with_3_chars 2 15000 3 15000
# generate_chart 500 comparing_algos_2_3_with_500_chars 2 10000 3 10000

# generate_chart 2 comparing_algos_2_4_with_2_chars 2 20000 4 20000
# generate_chart 2 comparing_algos_2_4_with_2_chars 2 40000 4 40000
# generate_chart 3 comparing_algos_2_4_with_3_chars 2 20000 4 20000
# generate_chart 500 comparing_algos_2_4_with_500_chars 2 15000 4 15000
# generate_chart 2 comparing_algos_2_4_with_2_chars_all_100k 2 100000 4 100000
# generate_chart 3 comparing_algos_2_4_with_3_chars_all_100k 2 100000 4 100000
# generate_chart 500 comparing_algos_2_4_with_500_chars_all_100k 2 100000 4 100000

# NEW CHARTS:
# generate_chart 2 150k_algos_2_3_with_2_chars 2 150000 3 150000
# generate_chart 3 150k_algos_2_3_with_3_chars 2 150000 3 150000
# generate_chart 500 150k_algos_2_3_with_500_chars 2 150000 3 150000
# generate_chart 10000 150k_algos_2_3_with_10k_chars 2 150000 3 150000
#
# generate_chart 2 150k_algos_2_4_with_2_chars 2 150000 4 150000
# generate_chart 3 150k_algos_2_4_with_3_chars 2 150000 4 150000
# generate_chart 500 150k_algos_2_4_with_500_chars 2 150000 4 150000
# generate_chart 10000 150k_algos_2_4_with_10k_chars 2 150000 4 150000
#
# generate_chart 2 150k_algos_4_5_with_2_chars 4 150000 5 150000
# generate_chart 3 150k_algos_4_5_with_3_chars 4 150000 5 150000
# generate_chart 500 150k_algos_4_5_with_500_chars 4 150000 5 150000
# generate_chart 10000 150k_algos_4_5_with_10k_chars 4 150000 5 150000
#
# generate_chart 2 150k_algos_3_5_with_2_chars 3 150000 5 150000
# generate_chart 3 150k_algos_3_5_with_3_chars 3 150000 5 150000
# generate_chart 500 150k_algos_3_5_with_500_chars 3 150000 5 150000
# generate_chart 10000 150k_algos_3_5_with_10k_chars 3 150000 5 150000

# python3 ./results/plot_summary.py ./results/charts
