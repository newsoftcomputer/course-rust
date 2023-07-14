
// Struct Classic

fn main() {
    // Classic struct with named fields
    struct Students { name: String, level: u8, remote: bool }
    let student = Students{name: String::from("Marco Giraldo"), level: 1, remote: true};
    println!("Student: {}, Level: {}, Remote: {}", student.name, student.level, student.remote);
}