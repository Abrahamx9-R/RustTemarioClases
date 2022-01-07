fn main() {
    /* enums*/
    //enum junto con el match son uno de los futures mas interesantes y poderosos que tiene rust
    //un enum es un tipo el cual almacena difetentes variantes
    //enum se define con CamelCase

    //en este ejemplo de enum se simulara la respuestas de un resvidor a una peticion por eso es Response
    enum Response {
        Success,//cuando la peticion fue correcta
        Error//cuadno la peticion manda error
    }

    //definimos una variable de tipo Success
    //para acceder a la opcion de un Enum es como se muestra a continuacion
    let respuesta=Response::Success;

    //ahora unimeros un enum con un match
    match respuesta {
        Response::Success=>println!("La peticion se realizo exitosamente"),
        Response::Error=>println!("No es posible completar la operacion")
    }


    //ademas podemos poner codigo de error, o anexar un valor en por ejemplo Error como se muestra a continuacion

    enum ResponseDos {
        Bien,
        Mal(u32),//puede almacenar 403,404,500 que son errores mas comunes
    }

    //con esto anexamos un valor entero a error en este caso 403
    let respuesta=ResponseDos::Mal(403);

    match respuesta {
        ResponseDos::Bien=>println!("La peticion se realizo exitosamente"),
        ResponseDos::Mal(403)=>println!("Forbidden"),
        ResponseDos::Mal(404)=>println!("Not found"),
        ResponseDos::Mal(405)=>println!("Internal server error"),
        ResponseDos::Mal(_)=>println!("No es posible completar la operacion")
    }

    //ademas se puede hacer lo siguiente
    /*

    enum ResponseDos {
        Bien,
        Mal(u32,String),//puede almacenar 403,404,500 que son errores mas comunes
    }

    //con esto anexamos un valor entero a error en este caso 403
    let respuesta=ResponseDos::Mal(501,String::from("No es posible completar la operacion"));

    match respuesta {
        ResponseDos::Bien=>println!("La peticion se realizo exitosamente"),
        ResponseDos::Mal(403,_)=>println!("Forbidden"),
        ResponseDos::Mal(404,_)=>println!("Not found"),
        ResponseDos::Mal(405,_)=>println!("Internal server error"),
        ResponseDos::Mal(_,mensaje)=>println!("{}",mensaje)
    }

    */
}
