
use std::io;
use std::io::BufRead;
use ansi_rgb::{ Foreground, blue, red };


fn main() {

    let a = String::from("OPTION LIST");

    println!("{}", "OPTION LIST".fg(blue()));
    println!("{}", "Select Language".fg(red()));
    println!("1-English");
    println!("2-Spanish");
    println!("0-Exit");

    loop {

        let stdin = io::stdin();

        
        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "0" {
            break
        }

        match action.as_ref() {
            "1" => english(),
            "2" => spanish(),
            _ => println!("Incorrect Option")
        }
    }
    
}


fn english() {

        println!("English language selected / Enter for continue");

        println!("Available Options");
        println!("1-Show file");
        println!("2-Create file");
        println!("3-Edit file");
        println!("4-Delete file");
        println!("0-Back");

        println!("Select an option");

    
    loop {
        
        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => println!("Show file selected"),
            "2" => println!("Create file selected"),
            "3" => println!("Edit file selected"),
            "4" => println!("Delete file selected"),
            "0" => main(),
            _ => println!("Incorrect option")
        }

        println!("Please select an option:");
    }
}

fn spanish() {
    println!("Lenguaje español seleccionado");

    println!("Opciones Disponibles");
    println!("1-Ver archivos");
    println!("2-Crear archivo");
    println!("3-Editar archivo");
    println!("4-Elimnar archivo");
    println!("0-Atras");


    loop {

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => println!("Mostrar archivos seleccionado"),
            "2" => println!("Creae archivo seleccionado"),
            "3" => println!("Editar archivo seleccionado"),
            "4" => println!("Eliminar archivo seleccionado"),
            "0" => main(),
            _ => println!("Incorrect option")
        }

        println!("Por favor seleccione una opcion:");
    }
}



