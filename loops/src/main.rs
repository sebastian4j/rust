fn main() {
    let mut counter = 0;

    loop {
        counter += 1;
        println!("loop loop");
        if counter == 10 {
            break;
        }
    }

    let res = loop {
        counter += 1;
        if counter == 20 {
            break counter * 3; // retorna el counter * 3
        }
    };

    while counter < 30 {
        println!("counter: {}", counter);
        counter += 1;
    }

    println!("counter: {} res: {}", counter, res);

    let a = [10, 11, 12, 13];
    let mut index = 0;
    while index < 4 {
        println!("(while) valor: {}", a[index]);
        index += 1;
    }

    for element in a.iter() {
        println!("(for) valor: {}", element);
    }

    for element in (0..4).rev() {
        println!("Range: {}", a[element]);
    }
    for element in (1..5).rev() {
        println!("Range rev: {}", element);
    }
}
