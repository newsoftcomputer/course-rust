
//Example using ownership, references and borrowing whit strings
//In this example we have to pass the values by reference 
use std::collections::HashMap;

fn main() {
    let my_key = String::from("My Key");
    let my_value = String::from("My Value");

    let mut hash = HashMap::new();
    //Here the HashMap is using the variables how value
    hash.insert(&my_key, &my_value);
    
    // Here the values have been copied and not moved because they are numeric
    println!("{:?}", hash);
    println!("{} {}", my_key, my_value);
}
