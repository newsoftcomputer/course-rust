
fn main() {

    // Declaration 01
    let a = [1, 2, 3, 4, 5];

    // Declaration 02
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"
              ];


    // Declaration 03
    let b: [i32; 5] = [1, 2, 3, 4, 5];

    
    // Declaration 04
    let c = [3; 5];     // out: let a = [3, 3, 3, 3, 3];

   
    // Declaration 05
    let d = [1, 2, 3, 4, 5];
    
    let first = d[0];
    let second = d[1];

    // Declaracion 06
    /// Inicializar un array de cadenas con valores vacíos: 
    /// Puedes crear un array de cadenas con un tamaño fijo y llenarlo con cadenas vacías. Por ejemplo:
    const EMPTY_STRING: String = String::new();
    let mut array: [String; 126] = [EMPTY_STRING; 126];

    
}