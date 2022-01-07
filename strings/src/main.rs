fn main() {
    /* strings*/
    //string cadena de caracteres, y en rust se pueden definir de dos formas
    //str es una cadena de caracteres inmutables
    //string es una cadena de caracteres inmutables
    //estos dos antgeriores se almacenan en dos lugares difernetes
    //str en stack
    //string en heap

    //definamos alguno de los definamos

    //str
    let variable_str="Hola, soy un tipo str";

    //string
    let variable_string=String::new();//esta es una cadena de caracteres vacia

    println!("El str es: {}",variable_str);
    println!("El string es: {}", variable_string);

    //pero tambien podemos definir un string a partir de un str como por ejemplo
    let variable_string=String::from("Hola, soy un string a partir de un str");
    println!("El string es: {}", variable_string);

    //ahora vamos a ver que mas cosas se peuden hacer con un string poderosas
    let mut variable_string=String::from("Hola, soy un string a partir de un str");

    //ahora colocaremos un caracter al final de la cadena con el metodo push() como vemos a continuacion

    variable_string.push(',');
    variable_string.push(' ');
    variable_string.push('H');
    variable_string.push('O');
    variable_string.push('O');
    variable_string.push('A');
    println!("El string es: {}", variable_string);

    //tambien podemos agregar en lugar de un caracter podemos agregar un str como veremos ahora
    variable_string.push_str(" anexando string");
    println!("El string es: {}", variable_string);

    //ademas podemos convertir un str a un string como veremos ahora, para ello se usa el metoro to_string()
    let nuevo_string="Hola, esto es una cadena".to_string();
    println!("{}", nuevo_string);

    //si queremos quitr una cadena se usa el siguiente metodo

    //falta

}
