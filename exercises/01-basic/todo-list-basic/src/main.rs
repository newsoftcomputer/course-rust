
use std::{
    io,
    io::{ BufRead, Write }
};


fn main() {

    let mut tasks: Vec<String> = vec![String::new(); 0];
    // let mut tasks: Vec<String> = vec![String::new(); 126];

    loop {

        println!("TODO LIST BASIC");
        println!("1: Show List Tasks");
        println!("2: Add Task");
        println!("3: Remove Task");
        println!("0: Exit");
        println!("Select a option:");

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();
        println!("Selected: {}", action);

        // if action == "0" {
        //     break
        // };

        match action.as_ref() {
            "1" => list_tasks(&tasks),
            "2" => add_task(&mut tasks),
            "3" => remove_task(&mut tasks),
            "0" => break,
            _ => println!("Invalid option {}", action)
        }

    }

}


fn list_tasks(tasks: &Vec<String>) {

    for task in tasks {
        println!(" - {}", task);
    }

    println!("Showed list tasks"); 

}


fn add_task(tasks: &mut Vec<String>) {

    let mut input = String::new();

    print!("Enter task: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let task = input.trim().to_string();

    tasks.push(task);
    
    println!("Task Added");

}


fn remove_task(tasks: &mut Vec<String>) {

    let mut input = String::new();

    print!("Enter task index to remove: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let index = input.trim().parse::<usize>().unwrap();

    if index >= tasks.len() {
        println!("Invalid index");
    } else {
        tasks.remove(index);
        println!("Task Removed");
    }

}

