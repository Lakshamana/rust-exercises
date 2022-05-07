mod random_numbers;
mod input;
mod debug;
mod ceasar;
mod primes;

fn main() {
    println!("6 random numbers: {:?}", random_numbers::generate_random_numbers(7, 0, 1100));
}
