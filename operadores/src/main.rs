fn main() {
    /*operadores*/
    let numero_uno:i32=10;
    let numero_dos:i32=200;

    //+-/*%
    let suma:i32=numero_uno+numero_dos;

    //relacionales tenemos >,<,>=,<=,==,!=
    let relacional:bool= suma != 2000;

    //operador or=|| y operador and=&&

    let resultado:bool=20+10>100 && true;

    println!("{} {} {} {} {}",numero_uno,numero_dos,suma,relacional,resultado);

}
