#[derive(Debug)] // se puede imprimir con {:?}
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // &mut self para modificar datos
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// puede tener multiples impl blocks
impl Rectangle {
    // associated functions: asociadas con la estructura pero no usan el &self
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    // opción 1
    let width = 30;
    let height = 50;

    println!("area 1: {}", area1(width, height));

    // opción 2
    let rect2 = (30, 50);
    println!("area 2: {}", area2(rect2));

    // opción 3
    let rect3 = Rectangle {width: 30, height: 50};
    println!("area 3: {}", area3(&rect3));

    println!("un struct: {:?}", rect3);
    println!("un struct formateado: {:#?}", rect3);
    println!("usando el método: {}", rect3.area());
    println!("can_hold?: {}", rect3.can_hold(&Rectangle {width: 20, height: 40}));
    let cuadrado = Rectangle::square(20);
    println!("cuadrado: {:?}", cuadrado);
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensiones: (u32, u32)) -> u32 {
    dimensiones.0 * dimensiones.1
}

fn area3(rectangle: &Rectangle) -> u32 { // solo la referencia, para no quedarse con la propiedad
    rectangle.height * rectangle.width
}