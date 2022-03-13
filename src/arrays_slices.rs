fn main() {
    // Type annotation is not required
    // Arrays length are known at compile time
    let array: [i32; 5] = [0, 1, 2, 3, 4];

    println!("array[0] = {}", array[0]);

    // This code will lead to error - index out of bounds error
    // println!("array[5] = {}", array[5]);
    
    println!("array len is {}", array.len());

    // assign values to non-mut arrays will lead to error - use `mut` arrays instead
    // array[0] = 6;
    // println!("Array now is {:?}", array);
    
    let mut first_five_fib = [1, 1, 2, 3, 4];
    first_five_fib[4] = 5u32; // note the unsigned value here

    // Get array size in bytes -- 5 * 4B = 20B
    println!("array size in bytes = {}", std::mem::size_of_val(&array));

    // borrow as a slice (slices length are only known at runtime)
    // pass as reference, aka borrow, like this
    slice_fn(&array);
    
    // Last lines can be left without final semicolon
    println!("Correct first_five_fib now is {:?}", first_five_fib)
    // println!("Range to vec: {:?}", (1..=5).map(|x| 1 + x).collect::<Vec<i32>>());
}

fn slice_fn(slice: &[i32]) {
    println!("First argument: {}", slice[0]);
}
