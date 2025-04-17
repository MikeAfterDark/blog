#!/bin/bash

data_output_folder="./results/data"
charts_output_folder="./results/charts"
program_path="./target/release/rust"

# input args
max_length=${1:-100}
iterations=20

# constants
num_chars=(2 3 $((max_length / 2)))
arr=(1 2)

cargo build --release

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
    shift
    local data_filenames=""

    while [[ $# -gt 1 ]]; do
        local algo=$1
        local length=$2
        shift 2

        data_filenames+=" ${data_output_folder}/${algo}_${length}_${char_count}.csv"
    done

    python3 ./plot_csv.py string_length success_avg_ns fail_avg_ns \
        "${char_count} chars" \
        "${charts_output_folder}/${char_count}_chars" \
        $data_filenames
}

# Syntax:
# generate_data algoIndex maxLength chars
# generate_data 1 1000 2
# generate_data 1 1200 3
# generate_data 1 1800 500

# generate_data 2 20000 2
# generate_data 2 15000 3
# generate_data 2 10000 500

# Syntax:
# generate_chart #chars algoIndex length algoIndex length... etc
# generate_chart 2 1 1000 2 20000
# generate_chart 3 1 1200 2 15000
# generate_chart 500 1 1800 2 10000
