
/* 

# TUPLES

    - Una tupla es una agrupación de valores de distintos tipos recopilados en un valor compuesto. Los valores individuales de una tupla se denominan elementos. Los valores se especifican como una lista separada por comas entre paréntesis (<value>, <value>, ...).

    - Una tupla tiene una longitud fija, que es igual a su número de elementos. Una vez declarada una tupla, no puede aumentar ni reducir su tamaño. No se pueden agregar ni quitar elementos. El tipo de datos de una tupla se define mediante la secuencia de los tipos de datos de los elementos.

    Definicion

    // Tuple of length 3
    
        let tuple_e = ('E', 5i32, true);


    
## Acceso a elementos de una tupla

    - Se puede acceder a los elementos de una tupla por la posición del índice, a partir de cero. Este proceso se conoce como indexación de tupla. Para acceder a un elemento de una tupla, usamos la sintaxis <tuple>.<index>.

    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);


*/


fn main() {


    // Sintaxis 1
    let tuple_e = ('E', 5i32, true);
        println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2); 
        // Out: Is 'E' the 5th letter of the alphabet? true


    // Sintaxis 2
    let tup: (i32, f64, u8) = (500, 6.4, 1);
        /* 
        The variable tup binds to the entire tuple because a tuple is considered a single compound element. 
        To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this: 
        */
        let (x, y, z) = tup;
        println!("The value of y is: {y}"); // Out: The value of y is: 6.4


    // Sintaxis 3
    // Declare tuple with types and assign value after
    let tuple_2: (i32, &str, f64, bool) = (123456, "Wallet1", 120.5, true);
        println!("i32: {}, &str: {}, f64: {}, Boolean: {}", tuple_2.0, tuple_2.1, tuple_2.2, tuple_2.3);

    
}

