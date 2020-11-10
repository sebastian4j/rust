#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

/*
es similar a definir varios tipos de struct pero sin usar el struct 
*/
#[derive(Debug)]
enum Message {
    Quit,                       // sin data asociada
    Move {x: i32, y: i32},      // struct anonimo
    Write(String),              // incluye un string
    ChangeColor(i32, i32, i32)  // incluye 3 valores i32
}

/* también se pueden definir metodos en enums */
impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
    V8(u8, u8, u8, u8)
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?} {:?}", four, six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let home2 = IpAddr2::V4(home.address);
    let loopback2 = IpAddr2::V6(loopback.address);

    println!("home2: {:?} loop: {:?}", home2, loopback2);
    let v8 = IpAddr2::V8(127, 0, 0, 1);
    println!("v8: {:?}", v8);

    Message::Write(String::from("write")).call();
    Message::ChangeColor(1, 2, 3).call();

    let movex = Message::Move{
        x: 1,
        y: 2
    };
    movex.call();
    Message::Move{x: 3,y: 4}.call();
    Message::ChangeColor(2,3,4).call();
}

fn route(ip_kind: IpAddrKind) {}
