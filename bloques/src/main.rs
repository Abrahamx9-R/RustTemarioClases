fn main() {
    /* Bloques*/
    //un bloque es una coleccion de sentencias que se encuentran agrupadas entre dos llaves
    //las variables solo existen en el bloque donde a sido definida

    let mensaje_uno ="Hola, soy una variable en el bloque main";

    //podemos crear un bloque de la siguiente manera

    {//este bloque al estar dentro del bloque main se le conoce como bloque hijo
        println!("Este es un segundo numero" );
        //como mensaje a sido definida en el bloque main tambien se puede usar en los bloques hijos
        println!("{}",mensaje_uno );
        //pero si definimos una variable en un bloque hijo o en otro bloque no se puede usar en bloques superiores o en otro que no este en este bloque
        let mensaje_dos="hola soy una variable en un bloque hijo";
        println!("{}",mensaje_dos );
        //ahora veamos que pasa si queremos hacer shadow a una variable que a sido definida en un bloque padre
        let mensaje_uno="hola soy una variable en un bloque hijo";
        println!("{}",mensaje_uno );
        //lo que pasa, si se puede observar, es que no hace un shadow a la variable, y esto es por que en este bloque no se ha definido una variable con el nombre de mensaje_uno, si no que se definio en el bloque padre por lo que podemos volverla a definir en este bloque y no destruye la variable del bloque superiores

    }

    println!("{}",mensaje_uno);

    /* bloques dos*/
    //algo importante que pueden hacer los bloques es retornar valores

    let resultado={//de esta forma se guarda el valor que retornara a una variable
        println!("Hola este es un bloque anidado" );
        let variable:i32=200;
        println!("{}",variable);
        //para retornar el valor a un bloque superiro solo se hace de la siguiente manera
        variable//aqui no estamos retornando la variable, si no su valor
    };// si asemos que el bloque retorne un valor y se guarde en una varible devemos de anexar al final un punto y coma

    println!("el valor del resultado es {}",resultado);

    //ahora un ejemplo bastante potente

    let calificaion:i8=10;

    let mensajes = if calificaion==10{
        //String::from() permite crear un string a partir de una cadena de caracteres pero strings se vera despues
        //no hace falta un punto y coma al final de la siguiente linea ya que es un valor que esta retornando el bloque
        String::from("Felicitaciones aprobaste la materia")
    }else{
        String::from("Necesitas estudiar mas")
    };

    println!("{}",mensajes );
}
