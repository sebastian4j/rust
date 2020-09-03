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
    // - se puede crear un String con nun string literal:
    let mut s = String::from("hola");
    s.push_str(" mundo!");
    println!("{}", s);

}
