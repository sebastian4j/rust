use std::io;

fn main() {
    let mut entrada = String::new();
    println!("ingrese en N buscado de la serie de fibonacci: ");
    io::stdin().read_line(&mut entrada).expect("no se puede leer la entrada");
    let entrada: u32 = entrada.trim().parse().expect("no es un número");
    let mut efes = [0u64, 1u64];

    let mut res = 0u64;

    if entrada == 0 {
        res = efes[0];
    } else if entrada == 1 {
        res = efes[1];
    } else {
        let mut cuenta = 0u32;
        while cuenta < (entrada - 1) {
            let n_1 = efes[1];
            let n_2 = efes[0];
            res = n_1 + n_2;
            efes[0] = n_1;
            efes[1] = res;
            cuenta += 1;
        }
    }
    println!("es: {}", res);
}
