//para definir nuestra estructura usamos struct y el nombre de la estrucura deve de tener formato camelcase
//ejemplo
struct User {
    username: String,
    password: String
}


fn main() {
    /* estructuras*/

    //definimos un nuevo objeto a partir de la estructuras
    let usuario=User{
        username:String::from("pepe"),
        password:String::from("hola123"),

        //ademas aquim, si tenemos en el bloque main asignado el valor previamente en una variable llamada username como se muestra a continuacion
        //let username=String::from("pepe");
        //y hacemos lo mismo con password, simplemente tenemos que poner en este bloque de codigo al definir usuario nuevo las variables username y password como se muestra a continuacion
        //username,password

    };

    println!("El nombre del usuario es: {}", usuario.username);
    println!("La contrasena del usuario es: {}", usuario.password);

    //algo importante que se debe saber es que al crear un objeto como con todas las variables, este sera inmutable por lo que si queremos que sea mutable devemos de colocar mut 

}
