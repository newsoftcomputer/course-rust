
# Tuplas

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