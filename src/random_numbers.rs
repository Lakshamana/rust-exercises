use rand::Rng;

pub fn generate_random_numbers(n: i32, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut result = Vec::new();

    for _ in 0..n {
        result.push(rng.gen_range(min..=max));
    }

    return result;
}

println!("6 random numbers: {:?}", generate_random_numbers(6, 0, 10));
