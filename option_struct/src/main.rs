struct User{
    username:String,
    password:String,
    email:String,
    edad:Option<u32>//aqui como la edad no es obligatoria podemos darle un option para que pueda o no almacenarce
}

fn main() {
    //ejemplo de como implementar el enum option 


    let usuario1=User{
        username:String::from("pepe"),
        password:String::from("password"),
        email:String::from("pepe@gmail.com"),
        edad:Some(26)//si no queremos que tenga edad basta con poner edad:None
    };
    //y con esto tambien podemos usar un unwrap o todo lo que ya habiamos visto

    match usuario1.edad{
        Some(edad)=>println!("Su edad es: {}",edad),
        None => { },
    }
}
