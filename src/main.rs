#![feature(is_sorted)]
use std::time;

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

fn main() {
    let n = 10_000;
    let runs = 5;
    println!("Std sort: \t{}", benchmark_sorting(n, runs, |x| x.sort()));
    println!("Bubble sort: \t{}", benchmark_sorting(n, runs, bubble_sort));
}
