
# BLEEDING / SANGRIA DEL CODIGO

    - En el cuerpo de la función, la mayoría de las instrucciones de código terminan con un punto y coma ;. Rust procesa estas instrucciones una tras otra, por orden. Cuando una instrucción de código no termina con un punto y coma, Rust sabe que la línea de código siguiente debe ejecutarse antes de que se pueda completar la instrucción inicial.

    Para ayudar a ver las relaciones de ejecución en el código, usamos la sangría. Este formato muestra cómo se organiza el código y revela el flujo de pasos necesarios para completar la tarea de la función. A una instrucción de código inicial se le aplica una sangría de cuatro espacios desde el margen izquierdo. Cuando el código no termina con un punto y coma, a la siguiente línea de código que se va a ejecutar se le aplica una sangría de cuatro espacios más.

    Veamos un ejemplo:

        fn main() { // The function declaration is not indented

            // First step in function body
                // Substep: execute before First step can be complete

            // Second step in function body
                // Substep A: execute before Second step can be complete
                // Substep B: execute before Second step can be complete
                    // Sub-substep 1: execute before Substep B can be complete

            // Third step in function body, and so on...
        }