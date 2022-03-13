fn main() {
    /*
     * added
       this
     * multiline comment section
     */
    println!("Using both {{ and }} to escape the caracters");

    // 31 in this case stands to the i32 type
    println!("A month can have up to {} days", 31);

    // You may specify the type by adding to literal numbers a suffix. E.g. i64
    // i64 would be used to represent big integer numbers
    println!("A month can have up to {} days", 31i64);

    // You can also provide a placeholder index
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // You can specify named arguments
    println!("{who} gave me {how_many} pences", who="You", how_many="3");

    // Special formatting
    println!("{} of {:b} people knows binary, the other half doesn't", 1, 2);

    // Padding with characters - 4 0s to the left
    println!("{number:0>width$}", number=1, width=5);

    // Padding with characters - 4 spaces to the left
    println!("{number:>width$}", number=1, width=5);

    // This code will produce an error
    // println!("My name is {1}, {0} {1}", "Bond");
    // A correct version would be like this:
    println!("My name is {1}, {0} {1}", "James", "Bob");

    // Debug - will display literally `"Loose"`
    println!("{what:?} yourself to dance!", what="Loose");
}
