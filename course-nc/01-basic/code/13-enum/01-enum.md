
# Uso de variantes de enumeración para datos compuestos

    - Las enumeraciones son tipos que pueden ser una de varias variantes. Lo que Rust denomina enumeraciones se conocen habitualmente como tipos de datos algebraicos. Lo importante es que cada variante de enumeración puede tener datos asociados.

    - Usamos la palabra clave enum para crear un tipo de enumeración, que puede tener cualquier combinación de las variantes de enumeración. Las variantes de enumeración, al igual que las estructuras, pueden tener campos con nombres, pero también los pueden tener sin nombres, o bien no tener ningún campo. Al igual que los tipos de estructura, los tipos de enumeración también se ponen en mayúsculas.

    - Definición de una enumeración
    En el ejemplo siguiente, se define una enumeración para clasificar un evento web. Cada variante de la enumeración es independiente y almacena diferentes cantidades y tipos de valores.

        enum WebEvent {
        // An enum variant can be like a unit struct without fields or data types
        WELoad,
        // An enum variant can be like a tuple struct with data types but no named fields
        WEKeys(String, char),
        // An enum variant can be like a classic struct with named fields and their data types
        WEClick { x: i64, y: i64 }

    - La enumeración de nuestro ejemplo tiene tres variantes de tipos diferentes:

    WELoad no tiene ningún tipo de datos o datos asociados.
    WEKeys tiene dos campos, con tipos de datos String y char.
    WEMClick incluye una estructura anónima con campos con nombre x y y, y sus tipos de datos (i64).
}