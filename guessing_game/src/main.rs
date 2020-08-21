// dejando a los tipos disponibles en el alcance
use std::io;
use rand::Rng;
use std::cmp::Ordering;
/*
A few number types can have a value between 1 and
100: i32, a 32-bit number; u32, an unsigned 32-bit number; i64, a 64-bit num-
ber; as well as others. Rust defaults to an i32
*/

// cargo doc --open

fn main() {
	let secret_number = rand::thread_rng() // obtengo el Rng
		.gen_range(1, 101); // entre 1 y 100
	println!("el número secreto: {}", secret_number);

	println!("Adivina el número secreto!");

	loop {

		// leyendo la entrada con un string
		println!("Ingresa el número: ");
		// let para crear una variable (inmutable x defecto)
		// mut para hacerla mutable
		let mut guess = String::new(); // associated function => llama al método estático new en el tipo String
		// sin el use xxx del inicio sería:
		// std::io::stdin()
		let result = io::stdin() // obtengo un Stdin
			// & es para indicar que es una referencia
			// referencia son inmutables x defecto
			.read_line(&mut guess) // invoco al método sobre la instancia obtenida
			.expect("Failed to read line"); // retorna un io::Result (un enum Ok o Err)
		// con el expect se muestra un mensaje si existe un error
		// y si está bueno retorna el resultado, en este caso son los bytes leidos

		// ahora obtener la entrada como un número
		// shadowing: se puede re usar el NOMBRE de la variable, al hacerlo oculta la anterior
		//   usado generalmente para redefinir los tipos de las variables como en este
		//   caso
		// - trim() borra los espacios y el \n final
		// - parse() convierte el string a algún tipo de número, hay
		//   que indicarle el tipo
		// - con los : se indica el tipo de guess
		// el u32 is un sin signo 32-bit integer, opción para pequeños números positivos
		let guess: u32 =
			/*
			// para el parseo con error, se cae el programa si se escribe un no número
			guess.trim().parse() // retorna un result
			.expect("tiene que ingresar un número");
			*/
			// para no cerrar el programa usar el match y no el expect
			match guess.trim().parse() { // retorna el result enum con Ok o Err
				Ok(num) => num, // fue exitoso el paso del string al number
				Err(_) => continue, // el string no es un numero
				// el _ es catch-all para atrapar todos los Err values
				// sin importar lo que tengan adentro
				// el continue pasará a la siguiente iteración del loop
			};

		println!("bytes leídos (uno x cada caracter + 2 x el \\n): {}", result);
		println!("Ingresado: {}", guess);


		// Ordering es otro enum

		// el 'match expression' es es para decidir que hacer basado
		// en la variante de ordering retornado
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("muy bajo!"),
			Ordering::Greater => println!("muy grande!"),
			Ordering::Equal => {
				println!("Ese es el número!");
				break;
			}
		}
	}
}
