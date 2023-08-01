
// Definiendo un vector de forma clasica

fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(29);
    vector.push(05);
    println!("{:?}", vector);
    vector.pop();
    println!("{:?}", vector);
}