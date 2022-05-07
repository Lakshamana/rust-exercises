const LIST: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNPQRSTUVWXYZ0123456789";

fn main() {
    println!("Ceasar: {}", ceasar("hola", "encipher"));
    println!("Ceasar indentity: {}", ceasar(&ceasar("hola", "decipher"), "decipher"));
}

fn ceasar(pass: &str, mode: &str) -> String {
    let factor = if Some(mode).unwrap_or_default() == "decipher" { 3 } else { -3 };
    pass
        .chars()
        .map(|c| {
            let len = LIST.len() as i32;
            let offset = (len + LIST.find(c).unwrap() as i32 + factor) % len as i32;
            LIST.chars().nth(offset as usize).unwrap()
        })
        .collect()
}
