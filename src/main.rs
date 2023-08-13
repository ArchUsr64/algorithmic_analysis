#![feature(is_sorted)]
#![feature(file_create_new)]
use std::{fs, io, time};

type Int = usize;

const OUT_PATH: &'static str = "data.csv";

fn quick_sort(arr: &mut [Int]) {
    if arr.len() == 1 {
        return;
    }
    match arr.first() {
        Some(first_element) => {
            let pivot = *first_element;
            let mut smaller_index = 1;
            let mut larger_index = arr.len() - 1;
            loop {
                if smaller_index > larger_index {
                    break;
                }
                if arr[smaller_index] > pivot && arr[larger_index] < pivot {
                    arr.swap(smaller_index, larger_index);
                }
                if arr[smaller_index] <= pivot {
                    smaller_index += 1;
                }
                if arr[larger_index] >= pivot {
                    larger_index -= 1;
                }
            }
            let pivot_new_index = larger_index;
            arr.swap(0, pivot_new_index);
            quick_sort(&mut arr[..=pivot_new_index]);
            quick_sort(&mut arr[pivot_new_index + 1..]);
        }
        None => return,
    }
}

fn quick_sort_better_locality(arr: &mut [Int]) {
    if arr.len() == 1 {
        return;
    }
    match arr.first() {
        Some(first_element) => {
            let pivot = *first_element;
            let mut smaller_portion = 1;
            let mut larger_portion = smaller_portion;
            loop {
                if larger_portion == arr.len() {
                    break;
                }
                if arr[smaller_portion] < pivot {
                    smaller_portion += 1;
                    larger_portion += 1;
                } else if arr[larger_portion] >= pivot {
                    larger_portion += 1;
                } else {
                    arr.swap(smaller_portion, larger_portion);
                }
            }
            let pivot_new_index = smaller_portion - 1;
            arr.swap(0, pivot_new_index);
            quick_sort_better_locality(&mut arr[..=pivot_new_index]);
            quick_sort_better_locality(&mut arr[pivot_new_index + 1..]);
        }
        None => return,
    }
}

fn bubble_sort(arr: &mut [Int]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..(len - 1 - i) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn merge_sort(arr: &mut [Int]) {
    fn merge(left: Vec<Int>, right: Vec<Int>) -> Vec<Int> {
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
    fn merge_recurse(arr: Vec<Int>) -> Vec<Int> {
        let len = arr.len();
        if len <= 1 {
            arr
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

fn lfsr_fib() -> Int {
    const START_STATE: i32 = 0x3A;
    static mut LFSR: i32 = START_STATE;
    let bit;

    unsafe {
        bit = ((LFSR >> 0) ^ (LFSR >> 2) ^ (LFSR >> 3) ^ (LFSR >> 5)) & 1;
        LFSR = (LFSR >> 1) | (bit << 15);
        return LFSR as Int;
    }
}

fn benchmark_sorting<F>(n: usize, runs: usize, algorithm: F) -> u128
where
    F: Fn(&mut [Int]) -> (),
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

fn bogo_sort(arr: &mut [Int]){
    loop{
        let i_1 = lfsr_fib() % arr.len();
        let i_2 = lfsr_fib() % arr.len();
        arr.swap(i_1, i_2);
        if (arr.is_sorted()) {
            break;
        }
    }
}

fn generate_report<T: io::Write>(
    output: &mut T,
    sample_points: &[usize],
    algorithm_name: &[&'static str],
    execution_times: &[Vec<u128>],
) {
    write!(output, "N").unwrap();
    algorithm_name
        .iter()
        .for_each(|name| write!(output, ",{}", name).unwrap());
    writeln!(output).unwrap();
    for (n, sample_times) in sample_points.iter().zip(execution_times.iter()) {
        write!(output, "{n}").unwrap();
        sample_times
            .iter()
            .for_each(|time| write!(output, ",{}", time).unwrap());
        writeln!(output).unwrap();
    }
}

fn main() {
    let output = &mut loop {
        match fs::File::create_new(OUT_PATH) {
            Ok(file_stream) => break file_stream,
            Err(e) => match e.kind() {
                io::ErrorKind::AlreadyExists => {
                    eprintln!("'{OUT_PATH}' already exists, will be removed!");
                    fs::remove_file(OUT_PATH).unwrap();
                    continue;
                }
                _ => panic!("{e:?}"),
            },
        }
    };
    let sample_points = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut execution_times = Vec::new();
    let algorithm_name = [
        "Unstable Sort",
        "Stable Sort",
        "Merge Sort",
        "Bubble Sort",
        "Quick Sort",
        "Quick Sort (Better Locality Partitioning)",
        "Bogo",
    ];
    let runs = 5;
    for n in sample_points {
        let temp_output = vec![
            benchmark_sorting(n, runs, |x| x.sort_unstable()),
            benchmark_sorting(n, runs, |x| x.sort()),
            benchmark_sorting(n, runs, merge_sort),
            benchmark_sorting(n, runs, bubble_sort),
            benchmark_sorting(n, runs, quick_sort),
            benchmark_sorting(n, runs, quick_sort_better_locality),
            benchmark_sorting(n, runs, bogo_sort),
        ];
        assert!(algorithm_name.len() == temp_output.len());
        execution_times.push(temp_output);
    }
    generate_report(output, &sample_points, &algorithm_name, &execution_times);
}
