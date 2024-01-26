
fn main() {
    
    // Ejemplo de que se esta cumpliendo la regla 1 y 2 del ownership
    let x = 10; // X es un owner y tiene un solo valor asignado (10)
    let y = 20; // Y es un owner y tiene un solo valor asignado (20)    

    println!("{}, {}", x, y);
    println!("{:p}, {:p}", &x, &y); // Imprimimos el puntero o referencia y podemos ver que son diferentes
}