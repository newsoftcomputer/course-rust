
# DATA TYPES


    - Rust es un lenguaje con establecimiento de tipos en modo estático. El compilador debe conocer el tipo de datos exacto de todas las variables del código para que el programa se compile y ejecute. Normalmente, el compilador puede inferir el tipo de datos de una variable en función del valor enlazado. No siempre es necesario indicar de forma explícita el tipo en el código. Cuando son posibles muchos tipos, debe informar al compilador del tipo específico mediante anotaciones de tipo.

    En el ejemplo siguiente, se le indica al compilador que cree la variable number como un entero de 32 bits. Especificamos el tipo de datos u32 después del nombre de la variable. Observe que después del nombre de la variable se usa el carácter de dos puntos :.

    - Los nombres de las variables se deben nombrar con snake_case


## Integer
    
    Length	Firmado	Sin signo	 	 
    8 bits	i8	u8	 	 
    16 bits	i16	u16	 	 
    32 bits	i32	u32	 	 
    64 bits	i64	u64	 	 
    128 bits	i128	u128
            
    dependiente de la arquitectura	isize	usize	 	 
    Los tipos isize y usize dependen del tipo de equipo en el que se ejecuta el programa. El tipo de 64 bits se usa en una arquitectura de 64 bits y el tipo de 32 bits, en una arquitectura de 32 bits. Si no especifica el tipo para un entero, y el sistema no puede deducir el tipo, asigna el tipo i32 (un entero de 32 bits con signo) de forma predeterminada.


    Examples: 

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

## Float

    - Rust tiene dos tipos de datos de punto flotante para los valores decimales: f32 (32 bits) y f64 (64 bits). El tipo de punto flotante predeterminado es f64. En las CPU modernas, el tipo f64 tiene aproximadamente la misma velocidad que el tipo f32, pero cuenta con una mayor precisión.

    Examples

        let number_64 = 4.0;      // compiler infers the value to use the default type f64
        let number_32: f32 = 5.0; // type f32 specified via annotation


## Boolean

    - El tipo booleano en Rust se usa para almacenar la veracidad. El tipo bool tiene dos valores posibles: true o false. Los valores booleanos se usan de forma generalizada en expresiones condicionales. Si una instrucción bool o un valor es true, realice esta acción; de lo contrario (la instrucción o el valor es false), realice una acción distinta. Una comprobación de comparación suele devolver un valor booleano.

    Ejemplo

        let status1: bool;
        let status2: bool = false;
        let status3 = false;

        status1 = false;


## Char

    - Rust admite valores de texto con dos tipos de cadena básicos y un tipo de carácter. Un carácter es un elemento único, mientras que una cadena es una serie de caracteres. Todos los tipos de texto son representaciones UTF-8 válidas.

    Ejemplo:

        let uppercase_s = 'S';
        let lowercase_f = 'f';
        let smiley_face = '😃';



## Cadenas de texto

    - El tipo str, también conocido como segmento de cadena, es una vista de los datos de la cadena. La mayoría de las veces, se hace referencia a estos tipos usando la sintaxis del estilo de referencia que precede al tipo con el símbolo de y comercial &str. Trataremos las referencias en los siguientes módulos. Por ahora, puede imaginarse &str como un puntero a datos de cadena inmutables. Los literales de cadena son todos de tipo &str.

    Aunque los literales de cadena son convenientes para usarlos en ejemplos de introducción de Rust, no son adecuados para todas las situaciones en las que podríamos querer usar texto. No todas las cadenas pueden conocerse en tiempo de compilación. Un ejemplo se da cuando un usuario interactúa con un programa en tiempo de ejecución y envía texto mediante un terminal.

    En estos escenarios, Rust tiene un segundo tipo de cadena denominado String. Este tipo se asigna en el montón. Cuando se usa el tipo String, no es necesario conocer la longitud de la cadena (número de caracteres) antes de compilar el código.

    - Si está familiarizado con un lenguaje de recolección de elementos no utilizados, es posible que se pregunte por qué Rust tiene dos tipos de cadena. 1 Las cadenas son tipos de datos extremadamente complejos. La mayoría de los lenguajes usan sus recolectores de elementos no utilizados para atenuar esta complejidad. Rust, como lenguaje de un sistema, expone parte de la complejidad inherente de las cadenas. La complejidad adicional conlleva una cantidad de control muy específica sobre cómo se usa la memoria en el programa.

    1 _En realidad, Rust tiene más de dos tipos de cadena. En este módulo, solo se describen los tipos String y &str. Puede obtener más información sobre los tipos de cadena que se ofrecen en la documentación de Rust.


### &str

    - 

    Examples:

        // Compiler interprets a series of items in quotations as a "str" data type and creates a "&str" reference
        let string_1 = "miley ";

        // Specify the data type "str" with the reference syntax "&str"
        let string_2: &str = "ace";


### String

    - The String
