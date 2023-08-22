
use std::collections::HashMap;

fn main() {
    let mut salas = HashMap::new();
    salas.insert("Javascripters".to_string(), 60);
    salas.insert("Rustaceans".to_string(), 80);
    println!("{:?}", salas);
}

