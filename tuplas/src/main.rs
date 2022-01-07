fn main() {
    /* tuplas*/

    //la principal diferencia entre una tuola y un arreglo es que en la tupla se pueden almacenar diferentes tipos de datos

    let tupla=(10, true, 5.5);
    //y todo funciona igual a un arreglo
    println!("el valor de la tupla es{:?}", tupla);

    //tambien se puede indicar que tipos de datos se guardaran en la tuplas, el problema es que lo tenemos que asignar para los n datos que queramos almacenar
    let tupla:(i32,bool,f64)=(10,false,5.5);

    //para acceder a un elemento de la tupla se puede hacer de la siguiente manera
    let elemento_uno=tupla.0;
    let elemento_ultimo=tupla.2;

    println!("el primer elemento de la tupla es {:?}",elemento_uno);
    println!("el ultimo elemento de la tupla es {:?}",elemento_ultimo);

    // modificar la tupla, de nuevo esta se tiene que definir como mutable
    let mut tupla=(10, true, 5.5);
    tupla.1=false;

    println!("el valor de la nueva tupla es {:?}",tupla );

    //otr diferencia es que al igual que los arreglos las tuplas no pueden incrementar ni disminuir su cantidad de elementos
}
