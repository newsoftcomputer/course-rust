
use std::{
    io,
    io::{ BufRead, Error, Read },
    fs::{ OpenOptions }
};

// use std::{
//     io,
//     in::lock,
// };

use std::*;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let mut action = String::new();
    let stdin = io::stdin();

    println!("GUESS THE NUMBER");
    println!("Game Language:");
    println!("1 => English");
    println!("2 => Spanish");
    println!("2 => Autor");

    println!("Que accion deseas realizar?");
        action = stdin.lock().lines().next().unwrap().unwrap();

    match action.as_ref() {
        "1" => println!("English"),
        "2" => println!("Spanish"),
        // "2" => create_todo(task, false),
        // "3" => println!("Completar tarea"),
        // "4" => println!("Eliminar tarea"),
        _ => println!("La opcion es invalida") 
    }

    if action == "1" {

        println!("Guess the number");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        // println!("The secret number is: {secret_number}");

        loop {

            println!("Please input your guess");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                // .expect("Please type a number!");
                Ok(num) => num,
                Err(_) => continue
            };
            


            println!("You guessed: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("YOU WIN!");
                    break;
                }
            }

        }

    } else {

        println!("Adivina el Numero");

        let secret_number = rand::thread_rng().gen_range(1..=100);
        // println!("El numero secreto es: {secret_number}");

        loop {

            println!("Por favor ingresa tu suposición");

            let mut guess = String::new();

            io::stdin()
                .read_line(&mut guess)
                .expect("FFallo al leer la linea");

            let guess: u32 = match guess.trim().parse() {
                // .expect("Please type a number!");
                Ok(num) => num,
                Err(_) => continue
            };
            


            println!("Tu suposicion es: {guess}");

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Muy bajo"),
                Ordering::Greater => println!("Muy alto"),
                Ordering::Equal => {
                    println!("GANASTE");
                    break;
                }
            }

        }

    }
    
}

