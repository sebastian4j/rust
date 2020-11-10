
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None; // hay que indicarle el tipo

    println!("{:?} {:?} {:?} ", some_number, some_string, absent_number);
    
    /*
    cuando un valor tenga un tipo no Option<T>
    se puede asumir en forma segura que el valor no es null

    los match son exhaustive: hay que cubrir todas las posibilidades
    para que el código sea valido.
    */
    match absent_number {
        None => {
            println!("no tiene un valor...");      
        },
        Some(i) => {
            println!("tiene un valor: {}", i);
        }
    };
    let res = sumas_uno(some_number);
    println!("sumar: {:?}", res);

    let x = Some(String::from("1"));
    let y = cambia_string(x); // x queda eliminado
    println!("{:?}", y);

    let numero = 1;
    /*
    - con el _ se pueden contener todos los casos que no se cubran
    - el orden SI importa, si se pone primero el _ no se pasará a los otros
     */ 
    match numero {
        1 => {
            println!("es 1")
        },
        _ => {
            println!("no es 1")
        }
    }
    /*
    if let permite evitar ocupar el _
     */ 
    if let Some(5) = some_number {
        println!("es cinco");
    }

    /*
    también puede tener else
    */
    if let Some(6) = some_number {
        println!("es seis");
    } else {
        println!("no es seis");
    }
}

fn sumas_uno(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn cambia_string(x: Option<String>) -> Option<String> {
    Some(String::from("qwerty"))
}
