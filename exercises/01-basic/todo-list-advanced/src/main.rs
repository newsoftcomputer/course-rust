
use std::io;
use std::io::BufRead;
use std::io::Write;
use ansi_rgb::{ Foreground, Background, blue, green, red, orange, cyan_blue, blue_magenta };
use rgb::RGB8;


fn main() {

    println!("");
    println!("{}", "TODO LIST ADVANCED".bg(cyan_blue()));
    println!("1: English");
    println!("2: Spanish");
    println!("{}", "0: Exit".fg(orange()));

    print!("Select language: ");
    io::stdout().flush().unwrap();

    loop {
        
        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        // println!("{}", "Selected: ".fg(gray700));
        // io::stdout().flush().unwrap();
        // println!("{}", action);

        match action.as_ref() {
            "1" => english_language(),
            "2" => spanish_language(),
            "0" => break,
            _ => println!("{}", "Invalid option".fg(red()))
        }

    }
}


/* TODO LIST ADVANCED - ENGLISH */

fn english_language() {

    let mut tasks: Vec<String> = vec![String::new(); 0];

    // Colors
    let _gray700 = RGB8::new(97, 97, 97);

    println!("{}", "English language seleted".fg(_gray700));
    println!("");

    loop {
        
        println!("");
        println!("{}", "TODO LIST ADVANCED - ENGLISH".fg(blue()));
        println!("1: List Tasks");
        println!("2: Add Task");
        println!("3: Edit Task");
        println!("4: Remove Task");
        println!("5: Cheked Task");
        println!("{}", "0: Back".fg(orange()));

        println!("");
        print!("Select a options: ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        // print!("{}", "Selected: ".fg(gray700));
        // io::stdout().flush().unwrap();
        // println!("{}", action);

        match action.as_ref() {
            "1" => list_tasks_english(&tasks),
            "2" => add_task_english(&mut tasks),
            "3" => edit_task_english(),
            "4" => remove_task_english(&mut tasks),
            "5" => check_task_english(&mut tasks),
            "0" => main(),
            _ => println!("{}", "Invalid option".fg(red()))
        }

    }

}

fn list_tasks_english(tasks: &Vec<String>) {

    println!("");

    for task in tasks {
        println!("- {}", task.fg(blue_magenta()));
    }

    println!("");

    println!("{}", "Listed Tasks".fg(green()));

}

fn add_task_english(tasks: &mut Vec<String>) {

    let mut input = String::new();

    println!("");
    print!("Enter task: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let task = input.trim().to_string();
    
    tasks.push(task);
    
    println!("{}", "Task added successfully".fg(green()));

}

fn edit_task_english() {

    println!("{}", "Task edit successfully".fg(green()));

}

fn remove_task_english(tasks: &mut Vec<String>) {

    let mut input = String::new();

    println!("");
    print!("Enter task index to remove: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        println!("{}", "Invalid task index".fg(red()));
    } else {
        tasks.remove(index);
        println!("{}", "Task removed successfully".fg(green()));
    }   

}

fn check_task_english(tasks: &mut Vec<String>) {

    println!("Enter the task ID to check");

}




/* TODO LIST ADVANCED - SPANISH */
fn spanish_language() {

    let mut tasks: Vec<String> = vec![String::new(); 0];

    // Colors
    let _gray700 = RGB8::new(97, 97, 97);

    println!("");
    println!("{}", "Lenguaje español seleccionado".fg(_gray700));

    loop {

        println!("");
        println!("{}", "TODO LIST AVANZADO - ESPAÑOL".fg(blue()));
        println!("1: Listar tareas");
        println!("2: Agregar tarea");
        println!("3: Editar tarea");
        println!("4: Eliminar tarea");
        println!("5: Chequear tarea");
        println!("{}", "0: Atras".fg(orange()));

        println!("");
        print!("Elija una opcion: ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        // println!("Opcion seleccionada: {}", action);

        match action.as_ref() {
            "1" => list_tasks_spanish(&tasks),
            "2" => add_task_spanish(&mut tasks),
            "3" => edit_task_spanish(&mut tasks),
            "4" => remove_task_spanish(&mut tasks),
            "5" => check_task_spanish(&mut tasks),
            "0" => main(),
            _ => println!("{}", "Opcion invalida".fg(red())) 
        }

    }

}

fn list_tasks_spanish(tasks: &Vec<String>) {

    println!("");

    for task in tasks {
        println!(" - {}", task.fg(blue_magenta()));
    }

    println!("");

    println!("{}", "Tareas listadas correctamente".fg(green()));

}

fn add_task_spanish(tasks: &mut Vec<String>) {

    let mut input = String::new();

    println!("");
    print!("Ingrese la tarea : ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let task = input.trim().to_string();

    tasks.push(task);

    println!("{}", "Tarea agregada correctamente".fg(green()));

}

fn edit_task_spanish(tasks: &mut Vec<String>) {

    let mut input = String::new();

    println!("");
    print!("Ingrese el ID de la tarea que desea editar: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();
    
    println!("La tarea selecciona es: {}", tasks[index]);

    let mut input_new = String::new();

    print!("Ingrese la nueva tarea: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input_new).unwrap();

    let _new_task = input_new.trim().to_string();

    // tasks.pu

    println!("{}", "Tarea editada correctamente".fg(green()));

}

fn remove_task_spanish(tasks: &mut Vec<String>) {

    let mut input = String::new();

    println!("");
    print!("Ingrese el ID de la tarea que desea eliminar: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        println!("{}", "ID invalido".fg(red()));
    } else {
        tasks.remove(index);
        println!("{}", "Tarea eliminada correctamente".fg(green()));
    }

}

fn check_task_spanish(tasks: &mut Vec<String>) {

    println!("Ingrese el ID de la tarea que desea chequear");

}

