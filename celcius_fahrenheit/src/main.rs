use std::io;

fn main() {
    loop {
        println!("\n::::::::::::::::::::::::::::::::::::::::::::::");
        println!("Fahrenheit a Celsius: F/f");
        println!("Celsius a Fahrenheit: C/c");
        println!("Salir: q/Q");
        println!("ingrese la opción: ");
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada)
            .expect("no se pudo leer la entrada");
        let entrada = entrada.trim();
        if entrada.eq("C") || entrada.eq("c") {
            println!("celcius a fahrenheit");
            mostrar(convertir_c_a_f(leer_numero()));
        } else if entrada.eq("F") || entrada.eq("f") {
            println!("fahrenheit a celcius");
            mostrar(convertir_f_a_c(leer_numero()));
        } else if entrada.eq("q") || entrada.eq("Q") {
            break;
        }
    }
}

fn mostrar(num: f64) {
    println!("resultado: {}", num);
}

fn leer_numero() -> f64 {
    println!("ingrese el valor: ");
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("no se puede leer número");
    entrada.trim().parse().expect("no es un número")
}

fn convertir_f_a_c(far: f64) -> f64 {
    (far - 32.0) / 1.8
}

fn convertir_c_a_f(cel: f64) -> f64 {
    cel * 1.8 + 32.0
}

