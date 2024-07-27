
use std::collections::HashMap;

fn main() {
    let mut salas = HashMap::new();

    // Valor repetido
    let value = salas.entry("Javascripters".to_string()).or_insert(100); 

    println!("{:?}", value);
}