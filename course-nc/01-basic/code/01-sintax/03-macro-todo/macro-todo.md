
# MACRO - TODO!

    - Macro todo! macro
    Cuando trabaje en los ejercicios de los módulos de Rust, observará que en el código de ejemplo se suele usar la macro todo!. En Rust, una macro es como una función y toma un número variable de argumentos de entrada. La macro todo! se usa para identificar código sin terminar en el programa de Rust. La macro es útil para crear prototipos, o bien cuando se quiere indicar un comportamiento que no está completo.

    Este es un ejemplo de cómo se usa la macro todo! en los ejercicios:

        fn main() {
            // Display the message "Hello, world!"
            todo!("Display the message by using the println!() macro");
        }


    - Al compilar código en el que se usa la macro todo!, el compilador puede devolver un mensaje de alarma en el que espera encontrar la funcionalidad completada:

        Compiling playground v0.0.1 (/playground)
        Finished dev [unoptimized + debuginfo] target(s) in 1.50s
        Running `target/debug/playground`
        thread 'main' panicked at 'not yet implemented: Display the message by using the println!() macro', src/main.rs:3:5
        note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace