
/*
Los enteros en Rust se identifican por el tamaño en bits y la propiedad signed. 
Un entero con signo puede ser un número positivo o negativo. 
Un entero sin signo solo puede ser un número positivo.

Los tipos isize y usize dependen del tipo de equipo en el que se ejecuta el programa. 
El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, 
en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, 
asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.
*/

fn main() {

    // Integers without signed
    let int_8bits: u8 = 5;
    println!("Integer Lenght: 8bits - Unsigned u8: {}", int_8bits);

    let int_16bits: u16 = 6;
    println!("Integer Lenght: 16bits - Unsigned u16: {}", int_16bits);

    let int_32bits: u32 = 80;
    println!("Integer Lenght: 32bits - Unsigned u32: {int_32bits}");

    let int_64bits: u64 = 200;
    println!("Integer Lenght: 64bits - Unsigned u64: {int_64bits}");

    // Lenght usize es dependiente de la arquitectura
    let int_size: usize = 200;
    println!("Integer Lenght: USize - Unsigned usize: {int_size}");



    // Integers with signed - Acepta + o -
    let int_8bits_positive: i8 = 45;
    println!("Integer Length: 8bits - Signed i8: {}", int_8bits_positive);

    let int_8bits_negative: i8 = -45;
    println!("Integer Length: 8bits - Signed i8: {}", int_8bits_negative);


    let int_16bits_p: i16 = 85;
    println!("Integer Length: 8bits - Signed i16: {int_16bits_p}");

    let int_16bits_n: i16 = -85;
    println!("Integer Length: 8bits - Signed i16: {int_16bits_n}");


    let int_32bits_p: i32 = 145;
    println!("Integer Length: 8bits - Signed i32: {int_32bits_p}");

    let int_32bits_n: i32 = 145;
    println!("Integer Length: 8bits - Signed i32: {int_32bits_n}");


    let int_64bits_p: i64 = 85;
    println!("Integer Length: 8bits - Signed i64: {int_64bits_p}");

    let int_64bits_n: i64 = 85;
    println!("Integer Length: 8bits - Signed i64: {int_64bits_n}");


    let int_128bits_p: i128 = 698;
    println!("Integer Length: 128bits - Signed: i128: {int_128bits_p}");

    let int_128bits_n: i128 = 698;
    println!("Integer Length: 128bits - Signed: i128: {int_128bits_n}");

    // Length ISize es dependiente de la arquitectura
    let int_size: isize = 458;
    println!("Integer isisze: undefined - Signed isize: {int_size}");
}