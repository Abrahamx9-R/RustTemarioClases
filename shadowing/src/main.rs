fn main() {
    //shadow es reutilizar el nombre de variables las veces que se necesiten

    let valor: i32 =10;
    //funcion macro println
    println!("el valor de la variable es {}",valor);
    // si intentamos modificar el valor de la variables no podemos por eso se usa el mut por eso se usa el concepto de shadow para cambiar valor a una variable inmutable
    let valor=20;
    println!("el valor de la variable ahora es{}",valor);
    //lo que pasa es que se destruye la primer variables que se llama valor y crear una nueva variables con el mismo nombre con el valor diferente
    //a esto se le conoce como shadow
    
}
