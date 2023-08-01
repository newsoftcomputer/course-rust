
use std::collections::HashMap;

fn main() {
    let mut salas = HashMap::new();
    salas.insert("Javascripters".to_string(), 60);
    salas.insert("Typescripters".to_string(), 70);
    salas.insert("Rustaceans".to_string(), 80);
    // Buscamos con get algun value almacenado previamente
    println!("{:?}", salas.get("Rustaceans"));
}

