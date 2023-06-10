
fn main() {

    //let guess: u32 = "42".parse().expect("Not a number!");

    // Integers
    let int8bits: i8 = 45;
    println!("Integer Length: 8bits - Signed: i8 - Unsigned: u8: {int8bits}");

    let int16bits: i16 = 85;
    println!("Integer Length: 8bits - Signed: i16 - Unsigned: u16: {int16bits}");

    let int32bits: i32 = 145;
    println!("Integer Length: 8bits - Signed: i32 - Unsigned: u32: {int32bits}");

    let int64bits: i64 = 85;
    println!("Integer Length: 8bits - Signed: i64 - Unsigned: u64: {int64bits}");

    let int128bits: i128 = 698;
    println!("Integer Length: 128bits - Signed: i128 - Unsigned: u128: {int128bits}");

    // let intsize: isize = 458;
    //println!("Integer isisze: undefined - Signed: isize - Unsigned: usize: {isize}");
}