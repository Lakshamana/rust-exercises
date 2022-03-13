mod arrays_slices;

fn main() {
    println!("6 random numbers: {:?}", arrays_slices::generate_random_numbers(6, 0, 10));
}
