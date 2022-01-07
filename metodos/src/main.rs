struct User {
    username:String,
    password:String,
}
//metodo es una funcion la cual le pertecene a una clase, este puede se utilizado para clases, estructuras, o por los objetos
//para implementar metodos en rust se usa impl y despues se define a que estructura se agragara los metodsos en este caso User
impl User {
    //algo importante es que todos los metodos deben de resivir el parametro self este hace referencia al objeto y se hace de la siguiente forma &mut self
    fn saluda(&mut self){
        println!("Hola, soy el usuario {}",self.username);
    }

    fn change_password(&mut self, new_password:String){
        self.password=new_password;
    }
}


fn main() {
    /* metodos */
    //para explicar esto se usara algo que ya se habia visto previametne como es struct para ello usaremos el ejemplo que se vio ahi
    let mut usuario_uno=User{
        username:String::from("pepe"),
        password:String::from("1234"),
    };
    println!("Hola, soy el usuario {}",usuario_uno.username);
    println!("Con contrasena {}",usuario_uno.password);

    //usando el metodo saluda
    usuario_uno.saluda();

    //usando el metodo new_password
    usuario_uno.change_password("4321".to_string());
    println!("Hola, soy el usuario {}",usuario_uno.username);
    println!("Con contrasena {}",usuario_uno.password);
}
