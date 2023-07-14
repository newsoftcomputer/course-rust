
# Definición de una enumeración con estructuras
    
    - Una manera de evitar los requisitos de variante de la enumeración es definir una estructura independiente para cada variante de esta. Después, cada variante de la enumeración usa la estructura correspondiente. La estructura contiene los mismos datos que tenía la variante de enumeración correspondiente. Este estilo de definición nos permite hacer referencia a cada variante lógica por sí misma.

    - En el código siguiente se muestra cómo utilizar este estilo de definición alternativo. Las estructuras se definen para contener los datos. Las variantes de la enumeración se definen para hacer referencia a las estructuras.