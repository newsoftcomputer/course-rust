
/* 
SHADOWING

 Puede declarar una variable nueva que use el nombre de una existente. La declaración nueva crea un enlace. En Rust, esta operación se denomina "propiedad reemplazada" porque la nueva variable prevalece sobre la anterior. La antigua variable sigue existiendo, pero ya no se puede hacer referencia a ella en este ámbito.

    En el código siguiente se muestra cómo usar la propiedad reemplazada. Declaramos una variable denominada shadow_num. No definimos la variable como mutable porque cada operación let crea una variable denominada shadow_num mientras se reemplaza la propiedad del enlace de la variable anterior.

    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let shadow_num = shadow_num + 5; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let shadow_num = shadow_num * 2; 

    println!("The number is {}.", shadow_num);

 */


fn main() {
    // Declare first variable binding with name "shadow_num"
    let x: i8 = 5;

    // Declare second variable binding, shadows existing variable "shadow_num" 
    let x: i8 = x + 7; 

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    let x: i8 = x - 2 * 8; 

    println!("The number is {}.", x);
}