#![feature(is_sorted)]
use std::{io, time};

fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn merge_sort(arr: &mut [i32]) {
    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let mut left = left;
        let mut right = right;
        while !(left.is_empty() || right.is_empty()) {
            result.push(if left.first().unwrap() < right.first().unwrap() {
                left.remove(0)
            } else {
                right.remove(0)
            })
        }
        while !left.is_empty() {
            result.push(left.remove(0));
        }
        while !right.is_empty() {
            result.push(right.remove(0));
        }
        result
    }
    fn merge_recurse(arr: Vec<i32>) -> Vec<i32> {
        let len = arr.len();
        if len <= 1 {
            arr
        } else if len == 2 {
            vec![arr[0].min(arr[1]), arr[1].max(arr[0])]
        } else {
            let left = arr[..len / 2].to_vec();
            let right = arr[len / 2..].to_vec();
            merge(merge_recurse(left), merge_recurse(right))
        }
    }
    let result = merge_recurse(arr.to_vec());
    arr.iter_mut()
        .enumerate()
        .for_each(|(i, value)| *value = result[i]);
}

fn lfsr_fib() -> i32 {
    const START_STATE: i32 = 0xAFD2;
    static mut LFSR: i32 = START_STATE;
    let bit;

    unsafe {
        bit = ((LFSR >> 0) ^ (LFSR >> 2) ^ (LFSR >> 3) ^ (LFSR >> 5)) & 1;
        LFSR = (LFSR >> 1) | (bit << 15);
        return LFSR;
    }
}

fn benchmark_sorting<F>(n: usize, runs: usize, algorithm: F) -> u128
where
    F: Fn(&mut [i32]) -> (),
{
    let mut times = Vec::with_capacity(runs);
    for _ in 0..runs {
        let mut test_input = Vec::with_capacity(n);
        for _ in 0..n {
            test_input.push(lfsr_fib());
        }
        let start = time::Instant::now();
        algorithm(&mut test_input);
        times.push(start.elapsed().as_nanos());
        assert!(test_input.is_sorted());
    }
    times.sort();
    // Return the median of the runs
    times[runs / 2]
}

fn generate_report<T>(
    output: &mut T,
    sample_points: &[usize],
    algorithm_name: (&'static str, &'static str),
    execution_times: &[(u128, u128)],
) where
    T: io::Write,
{
    writeln!(output, "{} vs {}", algorithm_name.0, algorithm_name.1).unwrap();
    for (n, (time_1, time_2)) in sample_points.iter().zip(execution_times.iter()) {
        writeln!(output, "{n}\t{time_1}\t{time_2}").unwrap();
    }
}

fn main() {
    let sample_points = [10, 100, 1000, 10_000];
    let mut execution_times = Vec::new();
    let algorithm_name = ("Merge sort", "Bubble sort");
    let runs = 3;
    for n in sample_points {
        let temp_output = (
            benchmark_sorting(n, runs, merge_sort),
            benchmark_sorting(n, runs, bubble_sort),
        );
        execution_times.push(temp_output);
    }
    generate_report(
        &mut io::stdout(),
        &sample_points,
        algorithm_name,
        &execution_times,
    );
}
