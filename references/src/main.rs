fn main() {
    let str = String::from("hola");
    let largo = calcular_largo(&str);
    println!("str: {} lrg: {}", str, largo);
    // con & se pasa hace referencia a un valor sin tomar su ownership
    // sin usar el & es dereferencing
    let mut mutable = String::from("str");
    println!("mutable: {}", mutable);
    referencia_mutable(&mut mutable);
    println!("mutable: {}", mutable);

    // no se puede tener mas de una referencia mutable en un
    // scope en particuar
    let m1 = &mut mutable;
    // let m2 = &mut mutable; // error de compilación xq m1 se ocupa abajo, si no se ocupara se podría definir
    m1.push_str(" mut ");
    println!("m1: {}", m1);

    {
        let m3 = &mut mutable;
        println!("m3: {}", m3);
    }

    let m2 = &mut mutable;
    let m4 = &mut mutable;
    let m5 = &mut mutable;
    //println!("{} {} {}", m2, m4, m5); // error de compilación xq se está usando m4 y m5 y son referencia mutables 
    /*
     todos podrian cambiar el string y rust evita el 'data race':
     - 2 o + punteros estan modificando la misma data
     - al menos un puntero se está usando para escribir data
     - no hay mecanismo usado para sincronizar la data
     */
    // como son referencia no mutables no tiene restricción
    let nomutable1 = &mutable;
    let nomutable2 = &mutable;
    println!("no mutable {} - {} ", nomutable1, nomutable2);

    // NO se puede tener una referencia mutable cuando existe una inmutable (no importa si se crea m7 primero y m6 después)
    // - explicación: si estoy ocupando una referencia inmutable y de pronto la referencia inmutable lo cambia no sería bueno
    let m6 = &mutable;
    // let m7 = &mut mutable;
    // println!("inmutable: {} mutable: {}", m6, m7); // acá se ocupa m6 y m7, se produce el error x cómo de crearon anteriormente
    let m8 = &mutable;
    println!("{} {} ", m6, m8); // m6 y m8 son inmutables, no hay problemas

    // rust asegura que no quedarán referencias perdidas/colgadas (dangling references)

    println!("no dangle {}", no_dangle());
}

// borrowing: tener las referencias como parametros de funciones
fn calcular_largo(s: &String) -> usize {
    s.len()
} // a referencia queda fuera del scope, pero nada pasa xq no
// tiene la propiedad de lo que refiere

fn referencia_mutable(s: &mut String) {
    s.push_str(" mutable ");
}

/*
no se puede usar porque se intenta retornar
una referencia a s
pero s será eliminada cuando termina
el scope de la función
*/
/*
fn dangle()  -> &String{
    let s = String::from("dangle");
    &s;
}
*/

/**
 * acá no hay problema con el valor retornado
 * pues se retorna el string y no 
 * la referencia
 */
fn no_dangle()  -> String{
    let s = String::from("no-dangle");
    s
}

/*

- en cualquier momento se puede tener uno de los siguientes (no ambos):
* una mutable
* cualquier numero de referencia no mutables

- las referencias siempre tienen que ser validas
*/