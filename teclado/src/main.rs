use std::io;//con esta bilbioteca se puede introducir por teclado valores de varialbes

fn main() {
    println!("ingresa un nombre de usario");
    let mut nombre= String::new();

    //read_line regresa un result, el cual puede tener un valor de exito o error
    io::stdin().read_line(&mut nombre); //se pasa la variable pro referencia y darle permiso de modificar los permisos se veran despues
    // la informacion se guarda en la varialbe junto con el salto de linea
    // para quitar el salto de linea se usa lo siguiente, usando shadows
    let nombre = nombre.trim();//el metodo trim elimina los saltos de linea

    println!("ingrese su edad ");

    let mut edad = String::new();

    io::stdin().read_line(&mut edad);

    let edad =edad.trim();
    //de igual forma regresa un #result por lo que se debe usar al final el metodo .unwrap()
    let edad: i32 =edad.parse().unwrap();

    println!("el valor de la variable es {} y su edad es {}",nombre,edad);


}
