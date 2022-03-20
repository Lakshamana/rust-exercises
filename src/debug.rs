#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{} months in a year", 12);

    println!("Now {:?} will be printed", Structure(7));

    // print Deep basic mode
    println!("Deep will be printed as {:?}", Deep(Structure(9)));

    // pretty printing
    println!("Deep will be printed as {:#?}", Deep(Structure(9)));

    // printing 9 only
    println!("Deep will be printed as {:?}", Deep(Structure(9)).0.0);

    let name = "John Doe";
    let age = 18;

    // notice, same as: let person = Person{ name: "John Doe", age: 18 };
    let person = Person{ name, age };

    println!("Person: {:#?}", person);
}
