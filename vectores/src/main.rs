fn main() {
    /* vectores*/
    //este es igual que un arreglo solo que la unica diferencia es que este si puede aumentar o disminuir su tamano, pero este solo puede almacenar un unico tipo de dato
    let vector=vec![1,2,3];

    //para trabajar con los elementos del vector es por indices

    println!("el valor del vector es {:?}",vector );

    //para agregar un elemento al final del vector usamos lo siguiente, pero primero devemos hacer a nuestro vector mutable

    let mut vector=vec![1,2,3];

    //agregar un elemento al final del vector
    vector.push(4);
    vector.push(5);

    //para agregar un elemento en una coordenada especifica se usa lo siguiente
    vector.insert(0,-1);
    vector.insert(1,0);

    println!("el nuevo valor del vector es {:?}",vector);

    //eliminar elementos, para eliminar el elemento devemos de dar a la funcion como valor la coordenada del elemento que queremos eliminar
    vector.remove(vector.len()-1);

    println!("el neuvo valor del vector es {:?}",vector );
    //para obtener algun elemento del vector podemos hacer los sigueinte
    let primer_elemento=vector[0];
    let ultimo_elemento=vector[vector.len()-1];

    println!("el primer elemento del vector es {:?}",primer_elemento );
    println!("el ultimo elemento del vector es {:?}",ultimo_elemento );

    //ademas podemos modificar el valor de algun indice
    vector[0]=10;

    println!("el nuevo valor del vector es {:?}",vector );

    //ademas podemos utilizar a los vectores como una pila con lo siguiente
    //este metodo pop nos permite obtener y eliminar el ultimo elemento del vector
    let ultimo_elemento=vector.pop().unwrap();

    println!("el ultimo elemento del vector es {:?}", ultimo_elemento);
    println!("el nuevo valor del vector es {:?}",vector );

    //otra forma de crear un vector, ya no utilizando la macro si no una estructura

    let mut vector= Vec::new();//se crea un vector basio, y cuando almacenamos el primer elemento, automaticamente todos los elementos deben de ser del mismo tipo
    //agregar un elemento al final del arreglo
    vector.push(1);
    vector.push(2);
    vector.push(3);
    vector.push(4);
    vector.push(5);

    println!("el vector generado de la nueva forma tiene valor de  {:?}",vector );

    //ademas podemos indicar el tipo de valores que se guardaran

    let mut vector:Vec<i32>=Vec::new();

    println!("este es un experimento {:?}",vector );
}
