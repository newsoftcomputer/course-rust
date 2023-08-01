
fn main() {
    // Declare a tuple of three elements with values directly
    let tuple_1 = ('E', 5, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {} the letter of the alphabet? {}", tuple_1.0, tuple_1.1, tuple_1.2);
    

    // Declare tuple with types and assign value after
    let tuple_2: (i32, &str, f64, bool) = (123456, "Wallet1", 120.5, true);
    
    println!("i32: {}, &str: {}, f64: {}, Boolean: {}", tuple_2.0, tuple_2.1, tuple_2.2, tuple_2.3);

}