
# FN MAIN

## Funciones en Rust

    - Una función es un bloque de código que realiza una tarea específica. Separamos el código de nuestro programa en bloques basados en tareas. Esta separación hace que el código sea más fácil de entender y mantener. Después de definir una función para una tarea, podemos llamar a la función cuando sea necesario realizar esa tarea.

    Cada programa de Rust debe tener una función llamada main. El código de la función main siempre es el primer código que se ejecuta en un programa con Rust. Podemos llamar a otras funciones desde la función main o desde otras funciones.

        fn main() {
            println!("Hello, world!");
        }

    - Para declarar una función en Rust, usamos la palabra clave fn. Después del nombre de la función, se le indica al compilador cuántos parámetros o argumentos espera la función como entrada. Los argumentos se enumeran entre paréntesis (). El cuerpo de la función es el código que realiza la tarea de una función y se define entre llaves {}. Un procedimiento recomendado consiste en aplicar formato al código para que la llave de apertura del cuerpo de la función aparezca justo después de la lista de argumentos entre paréntesis.

    