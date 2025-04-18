mod utility;

mod a1_simpletons_array;
mod a2_simpletons_counter;
mod a3_max_heap_my_ass;
mod a4_multithreading_a2;

use clap::{ArgGroup, Parser};
use core::panic;
use std::fs;
use rand::{Rng, rng};
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "No Adjacent Characters", 
    about = "Code implementation for blog", 
    version)]
#[command(group(
    ArgGroup::new("algorithm")
        .required(true)
        .args(&["algo", "all"])
))]
struct Args {
    /// [u32] Use a specific algorithm
    /// (mutually exclusive with --all)
    #[arg(long, value_name = "ALGO_INDEX")] 
    algo: Option<u32>,

    /// Run all algorithms in order with the same data
    /// (mutually exclusive with --algo)
    #[arg(long)]
    all: bool,

    /// [u128] [u128] Generate random input with a length and number of unique values
    #[arg(num_args = 2, value_names = ["LENGTH", "UNIQUE_COUNT"])]
    u128: Vec<u128>, 

    /// Output file
    #[arg(long)]
    output: Option<String>,

    /// include to have extra debug prints
    #[arg(long, default_value_t = false)]
    debug: bool,

    /// [u32] how many iterations to run the algorithms
    #[arg(long, default_value_t = 1)]
    iterations: u32, 

    /// include to increment from 0..N
    #[arg(long, default_value_t = false)]
    increment: bool,
}

type AlgoFn = fn(&[u128]) -> Vec<u128>;
const RUNNERS: &[AlgoFn] = &[
    a1_simpletons_array::run,
    a2_simpletons_counter::run,
    a3_max_heap_my_ass::run,
    a4_multithreading_a2::run,
];

#[derive(Debug, Clone)]
pub struct AlgoMetrics {
    pub name: String,
    pub result: Vec<u128>,
    pub duration: u128,
    pub valid: bool,
}

#[derive(Default)]
struct AlgoStats {
    name: String,
    success_durations: Vec<u128>,
    fail_durations: Vec<u128>,
}

fn main() {
    let config = Args::parse();
    let mut stats_map: HashMap<String, AlgoStats> = HashMap::new();

    let num_chars = config.u128[1];
    let start = if config.increment { config.u128[1] } else { 0 };
    let end = if config.increment { config.u128[0] } else { 1 };

    let mut results_summary = String::new();
    for i in start..end {
        let length = if config.increment { i } else { config.u128[0] }; 
        for _ in 0..config.iterations {
            let data = generate_int_vector(length, num_chars);

            let results = run_algorithms(&data, &config);
            for (name, duration_ns, result) in results {
                let success = !result.is_empty();
                record_stat(&mut stats_map, &name, duration_ns, success);
            }
        }
        results_summary += &generate_stats_report(&stats_map, length, num_chars);
    }

    write_output(&results_summary, config.output.as_ref());
}

fn generate_int_vector(length: u128, num_unique_chars: u128) -> Vec<u128> {
    if num_unique_chars < 1 {
        panic!("num_unique_chars cannot be less than 1");
    }
    if num_unique_chars > length {
        panic!("num_unique_chars cannot be greater than length");
    }

    let mut rng = rng();

    (0..length)
        .map(|_| rng.random_range(0..=num_unique_chars - 1))
        .collect()
}

fn run_algorithms(input: &[u128], args: &Args) -> Vec<(String, u128, Vec<u128>)> {
    if args.all {
        (0..RUNNERS.len())
            .map(|idx| run_algorithm(input, idx))
            .collect()
    } else {
        let algo_index = args
            .algo
            .and_then(|n| n.checked_sub(1))
            .filter(|&i| i < RUNNERS.len() as u32)
            .map(|i| i as usize)
            .unwrap_or_else(|| panic!("Invalid algo index: must be 1-{}", RUNNERS.len()));
        vec![run_algorithm(input, algo_index)]
    }
}


fn run_algorithm(input: &[u128], index: usize) -> (String, u128, Vec<u128>) {
    let (result, duration) = utility::measure(|| RUNNERS[index](input));
    let name = (index + 1).to_string();
    (name, duration.as_nanos(), result)
}

fn generate_stats_report(stats_map: &HashMap<String, AlgoStats>, string_length: u128, num_chars: u128) -> String {
    let mut stats_list: Vec<_> = stats_map.values().collect();
    stats_list.sort_by_key(|s| s.name.parse::<u32>().unwrap_or(u32::MAX));

    let mut report = String::new();

    for stats in stats_list {
        let total: Vec<u128> = stats.success_durations.iter()
            .chain(stats.fail_durations.iter())
            .copied()
            .collect();

        let avg = utility::average(&total);
        let min_val = utility::min(&total);
        let max_val = utility::max(&total);
        let std = utility::std_dev(&total);

        let success_avg = utility::average(&stats.success_durations);
        let fail_avg = utility::average(&stats.fail_durations);

        // headers in 'write_output()'
        report += &format!(
            "{},{},{},{},{},{},{:.2},{},{},{},{}\n",
            stats.name,
            string_length,
            num_chars,
            avg,
            min_val,
            max_val,
            std,
            success_avg,
            stats.success_durations.len(),
            fail_avg,
            stats.fail_durations.len()
        );
    }

    report
}
fn record_stat(
    stats_map: &mut HashMap<String, AlgoStats>,
    name: &str,
    duration_ns: u128,
    was_success: bool,
) {
    let stats = stats_map.entry(name.to_string()).or_insert_with(|| AlgoStats {
        name: name.to_string(),
        ..Default::default()
    });

    if was_success {
        stats.success_durations.push(duration_ns);
    } else {
        stats.fail_durations.push(duration_ns);
    }
}

fn write_output(output: &str, output_path: Option<&String>) {
    // data in 'generate_stats_report()'
    let headers = String::from("name,string_length,num_chars,avg_ns,min_ns,max_ns,std_dev,success_avg_ns,success_runs,fail_avg_ns,fail_runs\n");

    if let Some(path) = output_path {
        fs::write(path, headers + output).expect("Failed to write to output file");
    } else {
        println!("{}", headers + output);
    }
}
