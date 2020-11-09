/*
- stack y heap son partes de la memoria que están disponible para ser usadas
en runtime pero se estructuran en distintas formas

- el stack guarda valores en el orden que los obtiene y los remueve en el
orden opuesto = last in first out, el primero que entra es el primero que sale = una pila

- agregar data al stack es llamado pushing onto the stack

- remover del stack es llamado popping off the stack

- toda la data del stack tiene que tener un tamaño fijo y conocido

- la data que no se conoce su tamaño en compilación o que puede ser cambiado se guarda
en el heap

- el heap es menos organizado

- cuando se pone data en el heap se solicita espacio y el OS encuentra un
espacio vacío del heap que es suficiente, lo marca como siendo usado
y retorna un puntero el cual es la dirección de esa ubicación = allocation on the heap = allocation

- pushing values al stack NO se considera allocation x que el puntero
es conocido, de tamaño fijo

- pushing al stack es mas rápido que al heap xq el OS no tiene que buscar un lugar
para guardar la nueva data, la ubicación está siempre

## reglas del ownership

- cada valor en rust tiene una variable que se llama propietario (owner)

- solo un propietario a la vez

- cuando el owner queda fuera de alcance, el valor es descartado


 */

use std::ops::Add;

fn main() {
    // scope: rango dentro del programa en el cual un item es valido
    {   // s no es valido
        let s = "hola"; // string literal, no String
        println!("{}", s);
        // s es valida
    } // fin del scope que contiene a s
    // println!("{}", s); // s no es valida, fuera de scope (sin el {} sería valido)

    // - los signed/unsigned se guardan en el stack
    // - string literal es inmutable y se guarda en el stack
    // - String se guarda en el heap y permite guardar el string que es desconocido
    //   en tiempo de compilación y es mutable

    // - se puede crear un String con un string literal:
    let mut s = String::from("hola");
    s.push_str(" mundo!");
    println!("{}", s);


    let x = 5;
    let y = x;
    // y es una copia de y: como es un valor fijo y conocido en tiempo de compilación y vive en el stack se copia
    // cuando el tipo usa el trait Copy se mantienen las variables aún después de copiarlas (el antiguo al nuevo), pero
    // un tipo no puede usar el trait Copy si el o alguna de sus partes implementa el trait Drop, será un error de compilación
/*
     valores escalares simpes son Copy
     nada que requiera allocation es Copy
     con Copy:
     - todos os tipos enteros
     - los booleanos: bool
     - los caracteres: char
     - todos los tipos flotantes
     - tupas si solo contienen tipos que son Copy
*/

    let s1 = String::from("hello");
    let s2 = s1;
    /*
     un string esta hecho de 3 partes:
     - un puntero a la memoria que mantiene el contenido del string
     - largo actual del string en bytes
     - capacidad total de memoria en bytes que el string ha recibido del sistema operativo

     ese grupo de datos se guarda en el stack, el contenido del string se guarda en el heap

     - cuando se asigna s2 = s1 no se copia lo que está en el heap (el contenido del string)
       solo lo que está en el stack (puntero, largo y capacidad) == shallow copy
       PERO se invalida s1, no se puede seguir usando -  rust no creará una deep copy
*/
    // cuando una variable está fuera de alcance rust llama a la función 'drop' y a borra la variable del heap

    println!("string: {} x: {} y: {}", s2, x, y); // NO se puede usar s1, fue invalidada
    let s3 = s2.clone();
    println!("clonados: {} {} ", s2, s3); // al clonarlos se copia el heap y no se invalida s2

    let s4 = entregar_ownership();
    println!("s4: {}", s4);
    let s5 = obtiene_y_regresa(s4); // acá se pierde el s4, fue movido a 'obtiene_y_regresa'
    println!("nuevo: {}", s5); // s4 ya no existe

    let (s6, largo) = calcular_largo(s5); // asigno el string a s6 para no perder s5
    println!("el string recuperado: {} el largo: {}", s6, largo);

    let mut para_mutar = String::from("inicial");
    println!("valor original: {}", para_mutar);
    mutar_string(&mut para_mutar);
    println!("valor mutado: {}", para_mutar);
    
}

/** calcula el largo de un string. */
fn calcular_largo(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length) // retorno el string para no perderlo
}

/** creará un string y lo retornara entregando el ownership a quien lo llame */
fn entregar_ownership() -> String {
    let str = String::from("hola");
    println!("nuevo string creado: {}", str);
    str
}

fn obtiene_y_regresa(str: String) -> String {
    println!("string obtenido: {}", str);
    str
}

fn mutar_string(str: &mut String) {
    str.push_str(" -agregado- ")
}