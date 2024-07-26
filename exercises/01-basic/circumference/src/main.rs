
use std::io;
use std::io::{
    BufRead, Write
};
use ansi_rgb::{
    Foreground, blue, green, orange, red, yellow, cyan_blue, blue_magenta
}; 

fn main() {

    println!("");
    println!("{}", "CIRCUMFERENCE".fg(blue_magenta()));
    println!("{}", "Program to find the length of a circle");
    println!("");

    println!("{}", "e: Examples".fg(yellow()));
    println!("{}", "x: Exit".fg(orange()));

    print!("{}", "Add the radius of the circle in millimeters and press enter: ". fg(blue()));
    io::stdout().flush().unwrap();

    loop {

        let stdin = io::stdin();
        let action: i32 = 0; // = stdin.lock().lines().next().unwrap().unwrap();

        match action {
            10..=9999 => circle(action),
            1 => examples(),
            0 => break,
            _ => println!("{}", "Invalid option".fg(red()))
        }

    }

}


fn examples() {
    println!("");
    println!("Values Ok: 5, 10.0, 5.8");
    println!("Values Invalid : 5mm, 5,7");
    println!("");
    main();
}


fn circle(action: i32) {

    println!("");
    print!("{}", "Radio added: ".fg(cyan_blue()));
    println!("{}", action);

    const PI: f32 = 3.141592;
    let radio: f32 = action as f32; // = action.parse().unwrap();
    let circumference: f32 = 2.0 * PI * radio;

    print!("{}", "The length of circumferencia is: ".fg(cyan_blue())); 
    println!("{}", circumference.fg(green())); 

}

