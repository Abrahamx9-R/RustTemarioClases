struct Rectangulo {
    ancho:u32,
    alto:u32
}

fn area(rectangulo:Rectangulo)->u32{
    rectangulo.ancho*rectangulo.alto
}

fn main() {
    /* ownership*/
    //slash future o owneship es una caracteristica principal de rust y permite garantizar la seguridad de la memoria sin necesidad de usar un recolector de memoria

    let rectangulo=Rectangulo{ancho:10,alto:20};
    let resultado=area(rectangulo);

    println!("El area de rectangulo es {}",resultado);
    //ahora que pasa si queremos seguir usando nuestro objeto rectangulo, como por ejemplo imprimir lo sigueinte
//  println!("El ancho y alto del rectulo es: {} x {}", rectangulo.ancho,rectangulo.alto);
    //esto nos marcara un error que nos dice que el valor a sido prestado y por lo que ha sido movido

    //esto se debe al ownership y este tiene tres reglas
    //ownership reglas:
    /*
      *Cada valor en Rust tiene su propio ownership.
      *Solo puede existir un ownership a la vez
      *Si un ownership sale del alcance, valor se descartara
    */
    //ownership es como perteneciente a o le pertenece a, analicemos en el ejemplo anterior las reglas de ownership
    //primero veamos la regla numero 1
    //podemos ver que esta se cumple en la linea numero 14 ya que en esa linea se esta generando un nuevo objeto de tipo Rectangulo, y este le pertenece a la variable rectangulo (o esta alamcenado en la variable rectangulo)
    //ahora la regla numero dos, con esto sabemos que el nuevo objeto solo le pertenece a uno en este caso a la variable rectangulo
    //y la regla numero tres es un poco mas interesante
    //ya que como podemos ver el la linea numero 15 hacemos un llamado a la funcion area y estamos pasandole como argumento la variable rectangulo, pero en temas anteriores ya habiamos visto el alcance de las variables, que basicamente era el bloque donde habia sido definid, pero la regla 3 nos dice que si la variable sale de su alcance entocss el valor se descarta, y en Rust todos los argumentos que usemos en funciones o metodos se hacen por prestamos, en este caso al usar la variable rectangulo como argumento en la funcion area, esta se mueve, es decir deja de existir para main y ahora existe para area, ademas vimos que cuando un  bloque finaliza, todas las variables que hayan sido definidas se destruyen, es por eso que al final la linea numero 15, la variable rectangulo a sido destruida, y es por eso que marca el error de que a sido prestada y movida.

    //pero que los argumentos sean pasados por prestamos puede ser modificado como veremos a continuacio,
    //los argumentos tambien pueden ser pasados por referencia, para esto se pone un & antes del argumentos por ejemplo
    //definiendo la funcion
    /*
    fn area(rectangulo:&Rectangulo)->u32{
        rectangulo.ancho*rectangulo.alto
    }
    */
    //llamando a la funcion
    /*
    let resultado=area(&rectangulo);
    */

    //vemaos un ejemplo para entender mejor la regla 2
    //para ello veamos que pasa cuando hacemos los sigueinte
    let rectangulo=Rectangulo{ancho:10,alto:20};
    //aqui como ya habiamos comentado se cumple la regla nuemro 1
    let nuevo_rectangulo=rectangulo;
    //que pasa aqui?, bueno lo que pasa es que el ownership de rectangulo esta siendo tranferido a nuevo_rectangulo por lo que si ahora queremos usar la variable rectangulo, esto no podra ser posible ya que ahora el ownership de rectangulo lo tiene nuevo_rectangulo, y la regla numero dos nos dice que solo puede existir un ownership a la vez por lo que tendremos un error,
    //y esto pasa ya que estos son almacenados en Heap, pero si las variables son almacenadas en Stack, esto no pasa como vemos a continacion

    let x=10;
    let y=x;
    println!("{} {}",x,y);
}
