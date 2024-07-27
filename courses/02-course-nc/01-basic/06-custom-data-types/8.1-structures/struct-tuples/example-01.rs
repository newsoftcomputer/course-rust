
// Struct Tuple

fn main() {
    // Tuple struct with data types only
    struct Grades(char, char, char, char, f32);

    let mark_1 = Grades('A', 'A', 'B', 'A', 3.75);
    let mark_2 = Grades('B', 'A', 'A', 'C', 3.25);

    println!("Grades: {}, {}, {}, {}. Average: {}", mark_1.0, mark_1.1, mark_1.2, mark_1.3, mark_1.4);
    println!("Grades: {}, {}, {}, {}. Average: {}", mark_2.0, mark_2.1, mark_2.2, mark_2.3, mark_2.4);
}
