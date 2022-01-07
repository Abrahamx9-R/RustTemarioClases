fn main() {
    /* match */
    // match es equivalente a switch en otros lenguajes de programacion, en el cual se puede evaluar un valor en difentes casos
    let numero:i32=5;

    match numero {
        //se definen lo casos de la siguiente manera
        //valor=> sentencia /tarea,
        1=> println!("El numero es uno"),
        2=> println!("El numero es dos"),
        3=> println!("El numero es tres"),
        4|5|6=> println!("El numero esta entre el 4 y el 6"),
        7..=100=> {
            println!("El numero es mayor o igual al 7");
            println!("El numero se evalua mediante un rango")
        },
        _=> println!("{}",numero)// Este es el default en otros lenguajes de programacion
    }
    //ademas podemos usar el match para retornar valores como el siguiente ejemplo
    let mensaje= match numero {
        1=>"El numero es uno",
        2=>"El numero es dos",
        3=> "El numero es tres",
        4|5|6=> "El numero esta entre el 4 y el 6",
        7..=100=>"El numero se evalua mediante un rango",
        _=> "numero"// Este es el default en otros lenguajes de programacion
    };
    println!("{}",mensaje );

    //ejercicio fizz buzz

    for numero in 1..31{
        match (numero %3,numero%5) {//se esta evaluando sobre una tupla wow
            (0,0)=>println!("Fizz Buzz"),
            (0,_)=>println!("Fizz"),
            (_,0)=>println!("buzz"),
            (_,_)=>println!("{}",numero)//se puede sustituir (_,_) por simpelemente _
            //con esto nos damos cuenta de que en match no esta limitado a evaluar una variable simple como entero o char, si no cosas complejas como tuplas, arreglos, vectores, etc
            //algo muy poderoso
        }
    }

}
