
# SUBSTITUTING VALUES FOR {} ARGUMENTS

    - En las lecciones del módulo de Learn de Rust, a menudo llamamos a la macro println! con una lista de argumentos que incluye cadenas de texto con instancias de corchetes {} y otros valores. La macro println! reemplaza cada instancia de llaves {} dentro de una cadena de texto por el valor del argumento siguiente de la lista.

    Veamos un ejemplo:

        fn main() {
            // Call println! with three arguments: a string, a value, a value
            println!("The first letter of the English alphabet is {} and the last letter is {}.", 'A', 'Z');
        }

    Llamamos a la macro println! con tres argumentos: una cadena, un valor y otro valor. La macro procesa los argumentos por orden. Cada instancia de llaves {} dentro de una cadena de texto se reemplaza por el valor del argumento siguiente de la lista.

    La salida es la siguiente:

        The first letter of the English alphabet is A and the last letter is Z.