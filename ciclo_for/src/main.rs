fn main() {
    /* ciclo for*/
    //este nos permite iterar sobre una coleccion de datos y este funcionara como un ciclo for each, y este puede iterar en arreglos, vector, tupla, etc
    //definimos una arreglo donde vamos a iterar
    let numero:[i32;5]=[1,2,3,4,5];

    //en la siguiente linea el .iter() devuelve una estructura iterable
    for numero in numero.iter(){
        println!("el valor de numero: {}",numero );
    }

    //podemos hacer uso de un for junto con each

    for numero in 1..10{
        print!("{}",numero );
    }
    println!("");

    //algoritmo fizz buzz: si un numero es divisible entre 3 imprimimos fizz si un numero es divisible entre 5 imprimimos buzz, si un numero es divisible por 3 y 5 imprimimos fizz buzz y si un numero no es divisible entre 3 ni entre 5 no imprimimos nada

}
