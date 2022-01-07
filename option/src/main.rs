//este permite trabjar con cualquier tipo de dato, ya que utiliza la abstraccion T, y este posee dos opciones
/*
enum Option<T>{
    Some(T)->es una tupla que almacena cualquier tipo de dato aqui se almacena el calor que se quiera
    None->la ausencia de algun valor

}
*/
fn obtener_valor(bandera:bool)->Option<String>{
    if bandera{
        Some(String::from("mensaje para la tupla some"))
    }else{
        None
    }
    
}


fn main() {
    /* option*/
    //muchos lenguajes usas en null, el nill, o undefinded para representar la ausencia de algun valor, y exception para manejar los errores, pero en rust no exite,
    //en lugar de eso se usa dos tipos de existen
    //Option y
    //Result para errores y con ayuda de panic 
    
    //estructura del enum
    
    let resultado=obtener_valor(false);
    
    /*
    match resultado{
        Some(valor)=>println!("El valor es: {}",valor),
        None=>println!("No existe valor alguno")
    }
    */

    //tambien se puede hacer atravez de metodos como el 
    //unwrap el cual intenta obtener lo que la tupla Some almacena
    
    /*
    let valor_1=resultado.unwrap();
    println!("El valor es: {}",valor_1);//si no tiene un valor se ejecuta un panic
    */
    
    //existe tambien unwrap_or el cual nos deja poner un valor si es que no se tiene algun valor
    
    /*
    let valor_2=resultado.unwrap_or("La tupla no almacena ningun valor".to_string());
    println!("El  valor es: {}",valor_2)
    */
    
    //exite tambien .expect el cual intenta obtener lo que la tupla almacene, se ejecuta el panic con un valor que queramos que se imprima para especificar el error
    let valor_2=resultado.expect("se esperaba un valor");
    println!("El  valor es: {}",valor_2)

    
}
