use std::time::Instant;

fn sum_primes(n: usize) -> u32 {
    let mut primes = Vec::new();
    let mut current: u32 = 2;
    let mut sum: u32 = 0;

    while primes.len() < n {
        if primes.iter().all(|p| current % p != 0) {
            sum += current;
            primes.push(current);
        }
        current += 1;
    }
    sum
}

fn main() {
    let s = Instant::now();
    println!("{}", sum_primes(500));
    println!("duration: {:?}", s.elapsed());
}
