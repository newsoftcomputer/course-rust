
fn main() {
    let number_64 = 4.0;      // compiler infers the value to use the default type f64
    let number_32: f32 = 5.0; // type f32 specified via annotation

    println!("Float 64Bits: {}, Float 32Bits: {}", number_64, number_32);    
}