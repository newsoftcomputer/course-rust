
fn main() {
    
    let float_infe = 4.0;      // compiler infers the value to use the default type f64
    let float_32: f32 = 5.0; // type f32 specified via annotation
    let float_64: f64 = 3.0; // type f64 specified via casting
    
    println!("Float infere to 64Bits: {}, Float 32Bits: {}, Float 64Bits: {}", float, float_32, float_64);    

    /* 
    Experimental

    f16
    f128
    
    */
}