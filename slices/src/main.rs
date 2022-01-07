fn main() {
    /* slices*/
    //un slices es como un arreglo, la principal diferencia es que a los slice no se les define un tamano de alamacenamiento, es decir que se almacenan en el heap
    let mensaje=String::from("Hola mundo que pedo que hace");

    //definiendo un slice a partir de un string
    let hola=&mensaje[0..4];//se define como primero de que variable se obtendra, despues de donde empieza y hasta donde termina

    println!("{}", mensaje);
    println!("el slice es {}",hola );//lo mas comun es trabajar con strings los slices pero en general se pueden generar de otros valores existentes

}
