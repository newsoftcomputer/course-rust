
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


## Float


## Boolean

## Char

## &str

## String
