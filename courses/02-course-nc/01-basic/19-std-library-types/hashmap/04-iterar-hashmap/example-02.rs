
//Example using ownership, references and borrowing whit numbers

use std::collections::HashMap;

fn main() {
    let my_key = 10;
    let my_value = 20;

    let mut hash = HashMap::new();
    //Here the HashMap is using the variables how value
    hash.insert(my_key, my_value);
    
    // Here the values have been copied and not moved because they are numeric
    println!("{:?}", hash);
    println!("{} {}", my_key, my_value);
}
