
use std::collections::HashMap;

fn main() {
    let mut salas = HashMap::new();

    // Insertammos valores con insert al hashmap si hay 
    // algun valor repetido no lo agrega 
    salas.insert("Javascripters".to_string(), 60);
    salas.insert("Typescripters".to_string(), 70);
    salas.insert("Rustaceans".to_string(), 80);

    // Valor repetido
    salas.entry("Javascripters".to_string()).or_insert(100); 

    println!("{:?}", salas);
}