/*
una tuple struct es una tupla pero con nombre, sus propiedades no
serán nombradas
*/
struct Color(i32, i32, i32);
struct Point(f64, f64);

fn main() {
    let black = Color(0, 0, 0);
    let origen = Point(1.2, 3.4399);

/*
black y origin son de distinto tipo porque son de 
diferentes tuplas, cada una es de distinto tipo aunque 
ambos tengan los mismos tipos
*/

    println!("black: {} - origen: {}", black.0, origen.1);
}
