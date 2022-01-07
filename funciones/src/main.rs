/* funciones*/

//declaramos las funciones mediante snapecase es decir se separa por guion bajo

fn saludar_usuario(){
    println!("Hola desde una funcion")
}

//declaramos una funcion que recibe dos paramentos, y regresa un entero de 32 bits
//el valor que regresa es la ultima linea de codigo que este dentro del bloque de la funcion
fn suma(nunmero_uno:i32,numero_dos:i32)->i32{
    nunmero_uno+numero_dos
}

//ahora que pasa si necesitamos retornar un valor antes de que termine el bloque de codigo de la funcion
//bueno se debe utilizar ahora si la palabra reservada return como veremos ahora

fn factorial(numero:u32)->u32{
    if numero==1{
        return numero;//termina la funcion y regresamos el valor de 1
    }
    factorial(numero-1)*numero
}

fn main() {
    /* Funciones*/
    saludar_usuario();
    println!("{}",suma(10,20));
    println!("El factorial de 5 es {}",factorial(5));
}
