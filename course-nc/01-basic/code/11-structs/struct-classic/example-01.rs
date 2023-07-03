
fn main() {
    // Classic struct with named fields
    struct Students { name: String, level: u8, remote: bool }
    let student = Students{name: String::("Marco Giraldo"), level: 1, remote: true};
    println!("Studiantes {}", student.name);
}