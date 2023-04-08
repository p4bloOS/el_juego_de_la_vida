------------------------------------------------------------------------------
 el_juego_de_la_vida. Una aplicación multiplataforma hecha en Rust
 ----------------------------------------------------------------------------

Implementación gráfica del famoso autómata celular diseñado por John Horton Conway.

A partir de el proyecto base_egui ( https://github.com/p4bloOS/base_egui )

Programado en puro Rust.



INSTRUCCIONES DE COMPILACIÓN:

    Como único requisito debemos tener instalado docker con su demonio en ejecución (y git si queremos clonar este repositorio). Nos situaremos en el directorio "compilacion/" y ejecutaremos el script "compilar_con_docker.sh". Esto creará un contenedor que generará los ejecutables "zzzz" y "zzzzz" en el directorio "compilacion/", así como los archivos de la versión web en el directorio "compilacion/version_web/". La primera vez que se haga, el proceso podrá tardar varios minutos. En un futuro se podría ampliar fácilmente este proyecto para compilar a otras arquitecturas diferentes.

    $ git clone https://github.com/p4bloOS/zzzzzzzzzz
    $ cd zzzzzz/compilacion
    $ sudo sh compilar_con_docker.sh
 

INSTRUCCIONES PARA ARRANCAR SERVIDOR WEB:

    Como único requisito debemos tener instalado docker (y git si queremos clonar este repositorio). Nos situaremos en el directorio "servidor_web/" y ejecutaremos el script "arrancar_servidor_con_docker.sh". Esto creará un contenedor donde se ejecutará el servidor, que escuchará en el puerto 8080 de nuestro sistema (dicho puerto se puede cambiar fácilmente editando el script "arrancar_servidor_con_docker.sh").

    $ git clone https://github.com/p4bloOS/zzzzzzzz
    $ cd zzzzzzzz/servidor_web
    $ sudo sh arrancar_servidor_con_docker.sh



OTRAS CUESTIONES:

    - No estoy seguro de que funcione como debe cuando las células vivas sobrepasan los límites del tablero (he intentado que tenga un comportamiento toroidal)
