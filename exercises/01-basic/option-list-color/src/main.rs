
use std::io;
use std::io::BufRead;
use ansi_rgb::{ Foreground, Background, blue, red, green, orange };
use rgb::RGB8;


fn main() {

    println!("{}", "OPTION LIST".fg(green()));
    println!("1-English");
    println!("2-Spanish");
    println!("{}", "0-Exit".fg(orange()));
    println!("{}", "Select Language:".fg(blue()));

    loop {

        let stdin = io::stdin();

        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "0" {
            break
        }

        match action.as_ref() {
            "1" => english(),
            "2" => spanish(),
            _ => println!("{}", "Incorrect Option".fg(red()))
        }
    }
    
}


fn english() {

        // Colors
        let gray700 = RGB8::new(97, 97, 97);

        println!("{}", "English language selected ...".fg(gray700));
        println!("");

        println!("{}", "AVAILABLE OPTIONS".fg(green()) );
        println!("1-Show file");
        println!("2-Create file");
        println!("3-Edit file");
        println!("4-Delete file");
        println!("{}", "0-Back".fg(orange()));

        println!("{}", "Please select an option:".fg(blue()));
        
    loop {
        
        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        // Colors
        let gray700 = RGB8::new(97, 97, 97);

        match action.as_ref() {
            "1" => println!("{}", "Selected: Show file".fg(gray700)),
            "2" => println!("{}", "Selected: Create file".fg(gray700)),
            "3" => println!("{}", "Selected: Edit file".fg(gray700)),
            "4" => println!("{}", "Selected: Delete file".fg(gray700)),
            "0" => main(),
            _ => println!("{}","Incorrect option".fg(red()))
        }

        println!("");
        println!("{}", "Please select an option:".fg(blue()));
        
    }
}

fn spanish() {

    // Colors
    let gray700 = RGB8::new(97, 97, 97);

    println!("{}", "Lenguaje español seleccionado ...".fg(gray700));
    println!("");

    println!("{}", "OPCIONES DISPONIBLES".fg(green()));
    println!("1-Ver archivos");
    println!("2-Crear archivo");
    println!("3-Editar archivo");
    println!("4-Elimnar archivo");
    println!("{}", "0-Atras".fg(orange()));

    println!("{}", "Porfavor seleccione una opcion:".fg(blue()));


    loop {

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        // Colors
        let gray700 = RGB8::new(97, 97, 97);

        match action.as_ref() {
            "1" => println!("{}", "Seleccionado: Mostrar archivos".fg(gray700)),
            "2" => println!("{}", "Seleccionado: Crear archivo".fg(gray700)),
            "3" => println!("{}", "Seleccionado: Editar archivo".fg(gray700)),
            "4" => println!("{}", "Seleccionado: Eliminar archivo".fg(gray700)),
            "0" => main(),
            _ => println!("{}", "Opcion incorrecta".fg(red()))
        }

        println!("");
        println!("{}", "Por favor seleccione una opcion:".fg(blue()));
    }
}



