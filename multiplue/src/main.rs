fn main() {
    let start = 1;
    let end = 100_000;
    let num_threads = 4;

    let primes = find_primes_in_range(start, end, num_threads);
    println!("Found {} prime numbers.", primes.len());
}

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

fn find_primes_in_range(start: u64, end: u64, num_threads: usize) -> Vec<u64> {
    let range: Vec<u64> = (start..=end).collect();
    let pool = rayon::ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
    pool.install(|| {
        range.into_par_iter().filter(|&n| is_prime(n)).collect()
    })
}
