// - constantes
// no se puede usar mut con
// se pueden declarar en cualquier alcance
// solo se pueden asignar a una expresión constante y
// no a valores que se calculan en runtime (cómo el resultado de una función)

const MAX: u32 = 100_000;

/*
 - escalares:
 [1] integers

Length Signed Unsigned
8-bit i8 u8
16-bit i16 u16
32-bit i32 u32
64-bit i64 u64
128-bit i128 u128
arch isize usize

signed: −2^(n − 1) to 2^(n − 1) − 1 inclusivos, n es el numero de bits usados
i8: −(2^7) to 2^(7) − 1 = -128 a 127

unsigned: 0 to 2^(n) − 1, u8 = 0 to 2^8 - 1 = 0 a 255

Decimal         98_222
Hex             0xff
Octal           0o77
Binary          0b1111_0000
Byte (u8 only)  b'A'

cuando se compila en 'debug mode' se valida el valor máximo que pueden alcanzar
por el integer overflow que causa un panic en runtime, PERO cuando
se usa --release no se incluye ese check y el numero si es mayor al máxima
se regresa al comienzo: u8 es de 0 a 255, si recibe un 256 es 0, 257 es 1, ...



 [2] floating-point
 f32 (32 bits) y f64 (64 bits x defecto)
 IEEE-754 standard

 [3] Booleans
 one byte in size

 [4] characters
 4 bytes representando un unicode scalar value

el isize y el usize dependen del tipo de maquina que esté corriendo (64 bits o 32 bits)
*/

fn main() {
    println!("conceptos de programación con Rust");
    // al usar let se realiza la asignación, es posible hacer transformaciones
    // sobre el valor, pero cuando es asignado ya es inmutable
    let x = 5i8;
    let xx : i8 = 5;
    // x y xx son del mismo tipo y tienen el mismo valor
    // el byte literal NO permite un sufijo

    println!("xx {}", xx);
    println!("inmutable: {}", x);
    // x = 6; // no se puede x es inmutable
    let mut y = 6;
    println!("mutable es {}", y);
    y = 7;
    println!("mutable ahora es {}", y);
    println!("constante: {}", MAX);
    // y = "hola"; // no se puede, y es un i32
    // shadowing, no es lo mismo que mut
    let x = "8";
    println!("shadowing: {} {}", x, x.len());
    // punto flotante
    let f64 = 2.0;
    println!("punto flotante: {}", f64);

    let suma = 5 + 10;
    println!("suma: {}", suma);
    let resta = 95.5 - 4.3;
    println!("resta: {}", resta);
    let multiplica = 4 * 30;
    println!("multiplica: {}", multiplica);
    let divide = 56.7 / 4.0;
    println!("divide: {}", divide);
    let resto = 43 % 5;
    println!("resto: {}", resto);

    let ttrue = true;
    let ffalse: bool = false;
    println!("ttrue: {} ffalse: {}", ttrue, ffalse);

    let c = 'c';
    let z = 'z';
    let gato = '😻';
    println!(" {} {} {} ", c, z, gato);

    // la tupla permite agrupar valores de igual o distinto tipo
    // tiene largo fijo que se declara cuando se crea
    let tupla = (c, z, gato);
    let (v1, v2, v3) = tupla;
    println!("v1: {} v2: {} v3: {}", v1, v2, v3);
    let tupla = (c, suma, ttrue);
    println!("{} {} {}", tupla.0, tupla.1, tupla.2);
    let (v1, v2, v3) = tupla;
    println!("v1: {} v2: {} v3: {}", v1, v2, v3);

    // todos los elementos de un arreglo tienen el mismo tipo
    // tienen largo fijo y se guardan en el stack
    let arreglo = [1u8, 2, 3, 4]; // sin indicar el tipo
    println!("arr: {}", arreglo[0]);
    let arreglo : [u8; 3] = [1, 2, 3]; // indicando el tipo
    println!("arr: {}", arreglo[0]);
    let arreglo = [3; 5]; // tiene 5 tres
    println!("arr: {}", arreglo[0]);
    // acceder a un indice que no existe en un 'index out of bounds'
    //println!("arr: {}", arreglo[5]);
}
