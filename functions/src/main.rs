use rand::Rng;

fn main() {
    una_funcion();
    otra_funcion(23, "otro");
}

// puede definir parametros
// los valores concretos de los parametros son los argumentos
fn una_funcion() {
    println!("en una función");
}

// statements son instrucciones que realizan alguna acción y no retornan un valor
// expresiones evalúan a un valor resultante
fn otra_funcion(x: i32, y : &str) {
    println!("recibido: {} {}", x, y);

    let y = 6; // statement, no retorna un valor, no se puede asignar xel:
    // let x = (let y = 6);
    // la definición de la función es un statement
    println!("statemenet: {}", y);
    // el bloque usado para crear un scope es una expresión:
    let y = {
        let y = 3;
        y + 1 // expresiones no incluyen el ;
        // si se agrega el ; se convierte en un statement
    };
    println!("en scope: {}", y);

    println!("impl: {}", retorno_implicito(34));
    println!("expl: {}", retorno_explicito());
    let expl = retorno_implicito(34);
    println!("expl: {}", expl);
}

fn retorno_implicito(x: i32) -> i32 {
    5 + x + rand::thread_rng() // obtengo el Rng
        .gen_range(1, 101) // sin el ;
    // si tuviera el ; retornaría un () y sería un statement
    // y statement no retornan un valor y () representa una
    // tupla vacía
}

fn retorno_explicito() -> i32 {
    return 1 + 3 // funciona con y sin ;
}