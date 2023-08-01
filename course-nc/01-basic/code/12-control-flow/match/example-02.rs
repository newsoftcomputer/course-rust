
fn main() {
    use_match(1);
    use_match(6);
    use_match(25);
    use_match(31);
}

fn use_match(x: i32) {
    match x {
        1 => println!("Uno"),
        2 | 4 | 6 | 8 => println!("El numero es par"),
        20..=30 => println!("Entre 20 y 30"),
        _ => println!("No Found"),
    }
}