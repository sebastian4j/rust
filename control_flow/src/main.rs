fn main() {
    let number = 5;

    // el resultado de la expresión tiene que ser un bool o
    // será un error
    if number < 3 {
        println!("es menor que 3");
    } else if number == 3 {
        println!("es 3")
    } else {
        println!("es mayor a 3")
    }

    let cond = true;
    let number = if !cond {
        5
    } else if !true {
        6
    }
    // si no se pone es un error al compilar
    else {
        0
    };
    println!("{}", number);
}
