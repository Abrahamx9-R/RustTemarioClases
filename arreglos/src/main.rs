fn main() {
    /* arreglos */
    //se alamacenan los datos separados por comas
    //los inices comienzan de 0

    let arreglo_uno =[1,2,3,4,5];
    //:? es como decirle al compilador que muestre los valores que tiene el arreglo, si no los ponemos surgira un error al momento de compilar
    println!("el valor del arreglo uno es: {:?}",arreglo_uno );

    //ademas podemos definir que valores tendra nuestro arreglo en lugar de dejar que el compilador le de el valor, de las siguiente forma
    //cuando damos el tipo de dato que se almacenara en nuestro arreglo tambien tenemos que definir la cantidad de elementos que se tendra
    let arreglo_dos:[i32;5]=[1,2,3,4,5];
    println!("el valor del arreglo dos es {:?}",arreglo_dos );

    //tambien se peuden definir arreglos con valores por defecto
    let arreglo_tres=[5.5;10];
    //con esto estamos generando un arreglo de 10 elementos con un valor por defecto de 5.5
    println!("el valor del arreglo tres es{:?}",arreglo_tres);

    /* elementos de los arreglos*/
    //mostrando un elemento sabiendo su posicion

    let primer_elemento= arreglo_uno[0];
    //para saber el tamano de elementos en el arreglo tenemos la duncion .len() para cada arreglo
    let ultim_elemento= arreglo_uno[arreglo_uno.len()-1];
    println!("el ultimo elemento del arreglo uno es {:?}", ultim_elemento);
    println!("el ultimo elemento del arreglo uno es {:?}",primer_elemento);

    /* modificar elementos de arreglos*/

    //para modificar el arreglo, este se debe de definir como un arreglo mutable, ya que este por defecto se define como inmutable
    let mut arreglo_uno =[1,2,3,4,5];
    arreglo_uno[2]=30;

    println!("El nuevo arreglo uno ahora tiene valores de {:?}",arreglo_uno );

}
