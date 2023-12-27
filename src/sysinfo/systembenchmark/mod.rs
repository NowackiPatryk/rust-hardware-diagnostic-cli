use std::{
    thread::{self, available_parallelism, JoinHandle},
    time::SystemTime,
};
mod benchmark_methods;

pub fn process_benchmark() -> u128 {
    const BENCHMARKS_NUMBER: u128 = 100;
    let mut final_score: u128 = 0;

    let threads = setup_benchmark_threads(BENCHMARKS_NUMBER);
    let threads_number = threads.len() as u64;

    for thread in threads {
        let result = thread.join().unwrap();
        final_score += result / threads_number as u128;
    }

    final_score
}

fn setup_benchmark_threads(benchmarks_number: u128) -> Vec<JoinHandle<u128>> {
    let threads_number = (available_parallelism().unwrap().get()) as u64;
    let mut threads = vec![];

    for _ in 1..threads_number {
        let handle = thread::spawn(move || handle_thread_benchmark(benchmarks_number));

        threads.push(handle);
    }

    threads
}

fn handle_thread_benchmark(benchmarks_number: u128) -> u128 {
    let mut final_score: u128 = 0;

    for _ in 1..benchmarks_number {
        final_score += benchmark_cpu() / benchmarks_number;
    }

    final_score
}

fn benchmark_cpu() -> u128 {
    const FULL_SCORE: u128 = 1000;
    const EPOCHS: i32 = 10000000;

    let benchmark_start = SystemTime::now();

    for _ in 1..EPOCHS {
        benchmark_methods::square_benchmark();
    }

    let benchmark_end = SystemTime::now();
    let duration = calculate_duration(benchmark_start, benchmark_end);

    FULL_SCORE - duration
}

fn calculate_duration(start: SystemTime, end: SystemTime) -> u128 {
    end.duration_since(start).unwrap().as_millis()
}
