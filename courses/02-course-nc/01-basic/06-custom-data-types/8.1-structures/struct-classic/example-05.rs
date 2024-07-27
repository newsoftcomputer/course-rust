
// Sintaxis de actualizacion en estructuras
// Se realiza con dos puntos seguido de el nombre 
// de la instancia ...usuario1

#[derive(Debug)]
struct Usuario {
    user: String,
    email: String,
    time_active: u64,
    active: bool
}

fn main() {
    
    let usuario1 = Usuario {
        user: String::from("andresganc"),
        email: String::from("andresganc@gmail.com"),
        time_active: 1,
        active: true
    };

    /*
    Instanciamos Usuario en usuario2 y para no repetir
    Los campos que no cambian podemos poner .. seguido 
    de la instancia anterior y el antumaticamente completa
    los campos que hacen falta.
    */
    
    let usuario2 = Usuario {
        user: String::from("peluche"),
        email: String::from("mipeluche@gmail.com"),
        ..usuario1
    };

    println!("{:?}", usuario2);

}