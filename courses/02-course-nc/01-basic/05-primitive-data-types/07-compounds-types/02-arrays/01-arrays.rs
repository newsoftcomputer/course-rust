
/*

ARRAYS

Another way to have a collection of multiple values is with an array. 
Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, 
arrays in Rust have a fixed length.

*/


fn main() {

    // Sintaxis 01
        let a = [1, 2, 3, 4, 5];


    // Sintaxis 02
        /*
        Arrays are useful when you want your data allocated on the stack rather than the heap (we will discuss 
        the stack and the heap more in Chapter 4) or when you want to ensure you always have a fixed number of elements. 
        An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by 
        the standard library that is allowed to grow or shrink in size. If you’re unsure whether to use an array or a 
        vector, chances are you should use a vector. Chapter 8 discusses vectors in more detail.
        However, arrays are more useful when you know the number of elements will not need to change. 
        For example, if you were using the names of the month in a program, you would probably use an array rather 
        than a vector because you know it will always contain 12 elements:
        */
        let months = ["January", "February", "March", "April", "May", "June", "July",
                "August", "September", "October", "November", "December"
                ];


    // Sintaxis 03
        /*
        You write an array’s type using square brackets with the type of each element, a semicolon, 
        and then the number of elements in the array, like so:
        */
        let b: [i32; 5] = [1, 2, 3, 4, 5];

    
    // Sintaxis 04
        let c = [3; 5];     // Out: let a = [3, 3, 3, 3, 3];

   
    // Sintaxis 05
        /* 
        Accessing Array Elements An array is a single chunk of memory of a known, 
        fixed size that can be allocated on the stack. You can access elements of an array using indexing, like this:
        */
        let d = [1, 2, 3, 4, 5];
        let first = d[0];
        let second = d[1];

        
    // Sintaxis 06
        // Inicializar un array de cadenas con valores vacíos: 
        // Puedes crear un array de cadenas con un tamaño fijo y llenarlo con cadenas vacías. Por ejemplo:
        const EMPTY_STRING: String = String::new();
        let mut array: [String; 126] = [EMPTY_STRING; 126];

    
}