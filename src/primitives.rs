fn main() {
    // regular type annotation
    let boolean: bool = true; 
    let a_float: f64 = 1.0;

    println!("boolean={0}, a_float={1}", boolean, a_float);

    // suffix type annotation
    let an_integer = 5i32; // type = i32

    println!("an_integer={0}", an_integer);
    // default/inferred type annotation
    // by default variables are constant
    // In order to define them as actually variables, we use `mut` keyword
    let mut inferred = 12; // i32
    inferred = 67293726344i64; // i64
    inferred = 25;

    println!("inferred={0}", inferred);
    // The type, however, can't be changed (expects to receive either `i32` or `i64` values)
    // inferred = false;
    
    let inferred = true; // overwrite with shadowing using `let` keyword
    println!("shadowing inferred={0}", inferred);

    // Use underscores for better readability
    println!("One million can be written as {}", 1_000_000u32);

    // Use 0x, 0b and 0o -- you can use suffix in them too!
    println!("0011={}", 0b0011);
    println!("A={}", 0xau64);
    println!("0o76={}", 0o76u32);
s
