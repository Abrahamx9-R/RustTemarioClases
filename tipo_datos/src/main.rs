fn main() {
    /*    Tipo de Datos*/
    /*numeros naturales y enteros*/
    //i32 un entero de 32 bits pero se peude definir enteros de 8,16,32,64 y 128 bits y esto almacena numeros positivos como negativos->sig +-
    //si queremos almacenar numeros sin signo se puede usar
    //u8, y ademas de enteros con signo negativo o positivo, podemos definir estos de 8,16,32,64 y 128 bits->sig +

    let numero_uno:i8=-10;
    let numero_dos:u8=10;

    /*caracteres*/

    //char es un caracteres y se define con comillas simples
    let caracter='a';

    /*valores reales*/
    //al igual que los enteros se define con primero una f de float y luego el valor en bits que se quiere en la variable
    let real:f32=12.5;

    /*booleanos*/
    //los valores booleanos se definen con bool y se define su estado con minusculas
    let resultado:bool=false;//true

    //aqui se omiten las variables de tipo str y string
    println!("{} {} {} {} {}",numero_uno,numero_dos,caracter,real,resultado);
}
