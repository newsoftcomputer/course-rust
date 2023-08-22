
use std::{
    io,
    io::{ BufRead, Error, Read },
    fs::{ OpenOptions }
};

#[derive(Debug)]
struct Todo {
    task: String,
    done: bool
}

impl Todo {
    fn create(task: String, done: bool) -> Todo {
        Todo { task, done }
    }
}

fn main() {

    let todo: Vec<Todo> = all_todo().expect("Ha ocurrido un error al obtener la lista de tareas");
    println!("Tareas: {:?}", todo);

    loop {
        let mut action = String::new();
        let mut task = String::new();
        let stdin = io::stdin();

        println!("Acciones:");
        println!("1 => Mostrar tareas");
        println!("2 => Crear tarea");
        println!("3 => Completar tarea");
        println!("4 => Eliminar tarea");

        println!("Que accion deseas realizar?");
        action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "break" {
            break;
        };

        println!("Ingrese la tarea");
        task = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "1" => println!("Mostrar tareas"),
            "2" => create_todo(task, false),
            "3" => println!("Completar tarea"),
            "4" => println!("Eliminar tarea"),
            _ => println!("La opcion es invalida") 
        }
    }
}

fn create_todo(task: String, done: bool) {
    let ti = Todo::create(task.to_string() , false);
    println!("Tarea agregada: {}, Realizada: {}", ti.task, ti.done);
}

fn all_todo() -> Result<Vec<Todo>, Error> {
    let mut file = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .read(true)
                                    .open("todo.txt")
                                    .expect("Error al intentar abrir el archivo");
    let mut body = String::new();
    file.read_to_string(&mut body).expect("Error al intentar leer el archivo");
    let mut list = Vec<Todo> = Vec::new();

    for line in body.lines() {
        // Task:false
        let task = line.split(':').collect()::<Vec<$str>>();
        list.push(
            Todo::create(task[0].to_string(), task[1].parse().unwrap())
        );
    }

    Ok(list)
}
