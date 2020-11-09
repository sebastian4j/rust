fn main() {
    let mut str = String::from("hola mundo!");
    println!("first word: {}", first_word(&str));
    //
    let pos = index_first_word(&str);
    let pri = &str[0..pos];
    let mera = &str[..pos]; // se puede omitir si es 0
    let resto = &str[pos..]; // todo el resto del string sin el indice final

    let todo1 = &str[0..str.len()];
    let todo2 = &str[..str.len()];
    let todo3 = &str[0..];
    let todo4 = &str[..];

    println!("{} = {} //{}", pri, mera, resto);
    println!("{} = {} = {} = {}", todo1, todo2, todo3, todo4);
    println!("con literales: {} {}", 
        first_word_includes_literal("uno dos"), 
        first_word_includes_literal(&String::from("tres cuatro")));
    str.clear();

    let s = "hello world!";
    // s es de tipo &str -> un slice apuntando a ese especifico punto
    // por eso es inmutable -> &str lo es
    println!("{}", s);

    let a = [1, 2, 3, 4, 5];
    &a[1..3].iter();
}

/* busca la primera aparicion del espacio y retorna ese indice */
fn index_first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // string a array de bytes
    // el primer elemento del enumerate es el indice y 
    // el otro es la referencia al elemento
    for (i, &item) in bytes
        .iter() // iterator sobre los bytes - retorna cada elemento de la colexion
            .enumerate() { // envuelve el resultado de iter retorna cada elemento como parte de una tupla
        if item == b' ' { // byte literal syntax
            return i;
        }
    }

    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/*
sirve para &str y &String
*/
fn first_word_includes_literal(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}