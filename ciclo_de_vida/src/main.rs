fn main() {//bloque main-1
    /* ciclo de vida de las variavles */
    //primero definamos una variable
    let mensaje="Hola, soy una varaibles del bloque main";
    //sabemos, que esta variables solo puede ser utilizada en el bloque donde fue definida en este caso el bloque main y sus hijos

    println!("Bloque 1: {}",mensaje);

    {//bloque 2
        //que pasa si definimos una variable con el mismo nombre pero en el bloque 2
        let mensaje="Hola, soy una variable del bloque 2";
        println!("Bloque 2: {}", mensaje);

        {//bloque 3
            //que pasara aqui, que imprimira?
            println!("Bloque 3: {}",mensaje);

            //si definimos una variable en el bloque 3, esta no podra ser usada en bloques padre osea en este caso bloque 2 y 3
            //ya que todas las variables creadas en el bloque sera destruidas al terminal la variables
            let resultado=10+20;

        }
    }
    //por lo que en este caso el ciclo de vida de una variable depende de en que bloque sea definido

    //ahora veamos el ciclo de vida de scope pero con prestamos

    let mut mensaje=String::from("Hola, soy una variable para prestamo");

    {//bloque 2

        //let prestamo=mensaje;//sabemos por lo visto previamente que un prestamo quiere decir que la variable se mueve, es decir deja de existir en el bloque 1 o main para poder usarlo al bloque 2
        //si por ejemplo intentamos modificar la variable mensaje despues de ser prestada por ejemplo
        let let prestamo=&mensaje;
        mensaje=String::from("Cambio de valor");

        println!("{}",prestamo);
        //esto nos marcara un error ya que no podemos asignar un nuevo valor a la varaible mensaje, ya que la variable a sido prestada, a esto tambien se le llama freezing ya que estamos congelando la variable hasta que el bloque finalice
    }
}
