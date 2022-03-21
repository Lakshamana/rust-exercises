fn main() {
    println!("Ceasar: {}", ceasar("hola"));
}

fn ceasar(pass: &str) -> String {
    const LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ0123456789";

    pass
        .chars()
        .map(|c| {
            let len = LIST.len();
            LIST.chars().nth((LIST.find(c).unwrap() + len - 3) % len).unwrap()
        })
        .collect::<String>()
}
