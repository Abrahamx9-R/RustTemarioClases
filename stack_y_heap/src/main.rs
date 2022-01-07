fn foo(){
    let b=10;//se agrega al stack
    let c=20;//se agrega al stack
}

fn main() {
    /* stack y heap*/

    //son dos terminos en cuanto a manejo de memoria
    //son abstracciones que nos ayudan a entender en que parte de la memoria se encuentran almacenadas nuestras variables

    //stack se encuentran todas aquellas variables las cuales ya conocemos su longitud o tamano
    //por ejemplo
    let mut x:i32=10;
    //el stack tiene de ventaja que es mas rapido ya que al conocer el tamano de todas las varialbes se asignara el espacio de memoria mas optimas para ellas
    //y este se puede ver como una pila donde se almacenan los valores conforme se van agregando, un lifo
    //Esto hace a stack muy rapido en cuanto a escritura y lectura

    //veamos el siguiente ejemplo------------
    //-> leace primero la funcion foo

    let a=5;//de agrega al stack

    foo();
    //---------------------------------------
    //en ele ejemplo anterior se definireron tres variables, y si seguimos una forma secuencial de compilacion y ejecucion podemos ver que primero se define la variable a, y despues b y c, cuando la funcion foo finalizan las variables creadas ahi seran destruidas, y estas al estar en la parte superior de la pila son facilmente removidas evitando recalcular la pila


    //ahora si no conocemos el tamano de la variables estas se almacenan en el :
    //Heap, aqui se encuentran todas aquellas variables las cuales su tamano pueda cambiar su tamano ya sea incremente o decresca, un buen ejemplo de esto son los vectores o strings ya que se pueden agregar elementos o quitar.
    //a diferencia del stack es mas lento el heap en cuanto a escritura como a lectura.

}
