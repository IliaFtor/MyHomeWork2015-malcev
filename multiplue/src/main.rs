use rayon::prelude::*;
use std::time::Instant;

// Функция для проверки, является ли число простым
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// Функция для поиска простых чисел в заданном диапазоне
fn find_primes_in_range(start: u64, end: u64, num_threads: usize) -> Vec<u64> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    pool.install(|| {
        (start..=end).into_par_iter().filter(|&n| is_prime(n)).collect()
    })
}

fn main() {
    let start = 1;
    let end = 100_000;
    let thread_counts = [1, 2, 3, 4,5,6,7,8];

    for &num_threads in &thread_counts {
        let start_time = Instant::now();
        let primes = find_primes_in_range(start, end, num_threads);
        let duration = start_time.elapsed();
        println!("Threads: {}, Time: {:?}, Primes found: {}", num_threads, duration, primes.len());
    }
}
