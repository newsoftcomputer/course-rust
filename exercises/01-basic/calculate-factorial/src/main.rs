
use std::io;
use std::io::Write;

fn main() {

    println!("FACTORIAL");
    println!("El factorial es la multiplicacion de un numero por todos sus numeros previos y se expresa con el signo !");
    println!("Ejemplo: factorial de 5! = 5 x 4 x 3 x 2 x 1 = 120");
    
    
    println!("Agregue el numero que desea saber su factorial"); 
    print!("Numero: ");
    io::stdout().flush().unwrap();

    let result_factorial: u128 = calculate_factorial(25);
    println!("The factorial result from 25 is: {}", result_factorial);
}

fn calculate_factorial(num: u128) -> u128 {
    if num == 0 || num == 1 {
        1
    } else {
        let mut result: u128 = num;
        for i in (1..num).rev() {
            result = result * i;
        }
        return result;
    }
}
