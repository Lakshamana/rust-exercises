const THRESHOLD: u32 = 250_000;

fn is_prime(n: u32) -> u32 {
    if n <= 1 {
        return 0;
    }

    for i in 2..n / 2 {
        if n % i == 0 {
            return 0;
        }
    }

    1
}

fn main() {
    let mut num_primes = 0;
    for i in 2..THRESHOLD {
        num_primes += is_prime(i);
    }
    println!("{}", num_primes);
}
