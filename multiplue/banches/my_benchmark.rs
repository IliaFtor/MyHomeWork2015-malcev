use rayon::prelude::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
        if n % i == 0 || н % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

// Функция для поиска простых чисел в заданном диапазоне
fn find_primes_in_range(start: u64, end: u64, num_threads: usize) -> Vec<u64> {
    let range: Vec<u64> = (start..=end).collect();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    pool.install(|| {
        range.into_par_iter().filter(|&n| is_prime(n)).collect()
    })
}

// Функция для бенчмаркинга с использованием Criterion
fn bench_find_primes(c: &mut Criterion) {
    let start = 1;
    let end = 100_000;
    let thread_counts = [1, 2, 3, 4];

    for &num_threads in &thread_counts {
        c.bench_function(&format!("threads_{}", num_threads), |b| {
            b.iter(|| {
                let _primes = find_primes_in_range(black_box(start), black_box(end), num_threads);
            });
        });
    }
}

criterion_group!(benches, bench_find_primes);
criterion_main!(benches);
