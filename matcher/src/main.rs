enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        // pattern => code to run
        Coin::Dime => 10,
        Coin::Quarter(UsState::Alaska) => 67,
        Coin::Penny => {
            return 1;
        },
        Coin::Quarter(state) => {
            println!("estado: {:?}", state);
            45
        },
        Coin::Nickel => {
            34
        }
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
}
