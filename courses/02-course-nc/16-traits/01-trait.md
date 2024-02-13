
# TRAIT

    - Un rasgo define la funcionalidad que tiene un tipo determinado y que puede compartir con otros Tipos. Podemos usar los rasgospara definir el comportamiento compartido de una manera abstracta. Podemos Use límites de rasgos para especificar que un tipo genérico puede ser cualquier tipo que tenga cierto comportamiento.
    
    - Son metodos para un tipo de datos desconocido

    - Es parecido a las interfaces en Typescript

    Nota: Los rasgosson similares a una característica que a menudo se denomina interfaces en otros idiomas, aunque con algunas diferencias.


## Definicion de un rasgo

    - El comportamiento de un tipo consiste en los métodos a los que podemos llamar en ese tipo. Diferente Los tipos comparten el mismo comportamiento si podemos llamar a los mismos métodos en todos esos Tipos. Las definiciones de rasgos son una forma de agrupar las firmas de método para Definir un conjunto de comportamientos necesarios para lograr algún propósito.

    Por ejemplo, supongamos que tenemos varias estructuras que contienen varios tipos y Cantidades de texto: una estructura que contiene una noticia archivada en un ubicación particular y un que puede tener como máximo 280 caracteres a lo largo de con metadatos que indican si se trata de un nuevo tuit, un retuit o una respuesta a otro tuit.NewsArticleTweet

    Queremos crear una caja de biblioteca agregadora de medios llamada que pueda Mostrar resúmenes de los datos que pueden almacenarse en una instancia OR. Para ello, necesitamos un resumen de cada tipo, y solicitaremos ese resumen llamando a un método en una instancia. Listado 10-12 muestra la definición de un rasgo público que expresa este comportamiento.aggregatorNewsArticleTweetsummarizeSummary

        <>

        pub trait Summary {
            fn summarize(&self) -> String;
        }