//para definir una estructura tipo tupla se usa lo siguiente
#[derive(Debug)]//esta line de codigo permite a la estructura definida en la parte de abajo de esta, poder acceder a sus elementos con mayo facilidad unicamente impirmiendo el objeto creado con esta estructura
struct Color(u32,u32,u32);//la diferencia se encuentra, en que en una estructura normal esta se define con llaves y dentro de la s llaves, se definiran los elementos de esta, en este caso se define con parentecis, y dentro de los parentecis se definiran los valores que se almacenaran y de que tipo seran
//en el caso de esta estructura sabemos que la forma mas comun hoy en dia de identificar colores es con su convinacion de numeros para el RGB


fn main() {
    /*estructura tipo tupla*/

    //estructura tipo tupla, es una tupla que en lugar de poseer atrivutos poseera valores

    //usando nuestra estrucutura definida en la parte superior
    let negro=Color(0,0,0);
    let blanco=Color(255,255,255);

    println!("El color es: {:?}",negro);
    println!("El color es: {}, {}, {}",blanco.0,blanco.1,blanco.2 );
}
