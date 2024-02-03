
use std::io;
use std::io::BufRead;

fn main() {

    println!("TODO LIST BASIC");
    println!("New Task");
    println!("Show Task");

    loop {
        
        let stdin = io::stdin();

        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "0" {
            break
        }

        println!("Value: {}", action)

    }

}


fn showTasks() {

    println!("Show task"); 

}

fn addTask() {
    
    println!("Added Task");

}

