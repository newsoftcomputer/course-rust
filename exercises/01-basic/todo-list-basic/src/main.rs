
use std::io;
use std::io::BufRead;




fn main() {

    let mut vector: Vec<String> = vec![String::new(); 126];

    loop {

        println!("TODO LIST BASIC");
        println!("1: Show Task");
        println!("2: New Task");
        println!("0: Exit");
        println!("Select a option:");

        let stdin = io::stdin();
        let action = stdin.lock().lines().next().unwrap().unwrap();
        println!("Selected: {}", action);

        if action == "0" {
            break
        };

        match action.as_ref() {
            "1" => show_tasks(),
            "2" => new_task(),
            _ => println!("Invalid option")
        }

    }

}


fn show_tasks() {

    println!("Showed task"); 

}

fn new_task() {
    
    println!("Added Task");

}

