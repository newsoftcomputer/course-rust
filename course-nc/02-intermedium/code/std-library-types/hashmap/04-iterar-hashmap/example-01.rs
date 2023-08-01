
use std::collections::HashMap;

fn main() {
    let mut salas = HashMap::new();
    salas.insert("Javascripters".to_string(), 100);
    salas.insert("Typescripters".to_string(), 80);
    salas.insert("Rustaceans".to_string(), 20);

    for (key, value) in &salas {
        println!("Key: {} - Value: {}", key, value);
    }
}