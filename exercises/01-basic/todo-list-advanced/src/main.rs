
use std::io;
use std::io::BufRead;


fn main() {

    let mut tasks: Vec<String> = vec![String::new(); 0];

    println!("TODO LIST ADVANCED");
    println!("1: English");
    println!("2: Spanish");
    println!("0: Exit");
    println!("Select language"); 

    loop {
        
        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        println!("Selected: {}", action);

        match action.as_ref() {
            "1" => english_language(),
            "2" => spanish_language(),
            "0" => break,
            _ => println!("Invalid option: {}", action)
        }

    }
}


/* TODO LIST ADVANCED - ENGLISH */

fn english_language() {

    println!("English language seleted");

    loop {
        
        println!("TODO LIST ADVANCED");
        println!("1: List Tasks");
        println!("2: Add Task");
        println!("3: Edit Task");
        println!("4: Remove Task");
        println!("0: Back");

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        println!("Selected: {}", action);

        match action.as_ref() {
            "1" => list_tasks_english(),
            "2" => add_task_english(),
            "3" => edit_task_english(),
            "4" => remove_task_english(),
            "0" => main(),
            _ => println!("Invalid option: {}", action)
        }

    }

}

fn list_tasks_english() {

    println!("List Tasks");

}

fn add_task_english() {

    println!("Add Task");

}

fn edit_task_english() {

    println!("Edit Task");

}

fn remove_task_english() {

    println!("Remove Task");

}


/* TODO LIST ADVANCED - SPANISH */
fn spanish_language() {

    println!("Seleccionado el lenguaje español");

    loop {

        println!("TODO LIST AVANZADO - ESPAÑOL");
        println!("1: Listar tareas");
        println!("2: Agregar tarea");
        println!("3: Editar tarea");
        println!("4: Eliminar tarea");
        println!("0: Atras");

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();

        println!("Opcion seleccionada: {}", action);

        match action.as_ref() {
            "1" => list_tasks_spanish(),
            "2" => add_task_spanish(),
            "3" => edit_task_spanish(),
            "4" => remove_task_spanish(),
            "0" => main(),
            _ => println!("Opcion invalida") 
        }

    }

}

fn list_tasks_spanish() {

    println!("Listar tareas");

}

fn add_task_spanish() {

    println!("Agregar tarea");

}

fn edit_task_spanish() {

    println!("Editar tarea");

}

fn remove_task_spanish() {

    println!("Eliminar tarea");

}

