
// Definir un vector con macros

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    println!("{:?}", vector);
    vector.push(7);
    vector.push(8);
    println!("{:?}", vector);
    vector.pop();
    println!("{:?}", vector);
}

