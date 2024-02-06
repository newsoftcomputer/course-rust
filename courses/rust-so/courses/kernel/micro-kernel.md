
# MICRO KERNEL

    - Principio de minimalismo de Liedtke

        Ex: Un elemento es aceptable dentro del kernel solamente si al retirarlo de este pierde funcionalidad

    
    - Kernel que provee los mecanismos basicos

        * Manejo de memoria

        * Manejo de procesos e hilos

        * InterProcess Communication (IPC)


    - Todos los demas servicios como ( Drivers, Protocols de red, File system ) no forman parte del kernel pero si del sistema operativo y se ejecutan en espacio de usuario.

        * Por lo tanto las eventuales fallas en un servicio no afectan al kernel

        * Se puede garantizar que el kernel pueda tener una mayor estabilidad y no se vea afectado por los otros servicios externos.

    
    
    - IPC (Interprocess communication): Mecanismo para que los pracesos se comuniquen entre ellos


        * Se realizan generalmente por medio de mensajes

        * Al tener esta caracteristica de comunicacion por mensajes se establece un tipo de arquitectura Cliente - Servidor dentro del mismo sistema
        
        * Comunicacion Sincrona: Se envia el mensaje y se espera la respuesta





## Ejemplos de kernels hibridos


    - PC's

        * 