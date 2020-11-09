struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    // los que tienen el mismo nombre se pueden poner una sola vez (field init)
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    /*
    toda tiene que ser mutable si se quiere cambiar
    algún valor, no pueden ser algunos campos
    */
    let mut user1 = User {
        email: String::from("sebastian4j@gmail.com"),
        username: String::from("sebastian4j"),
        active: true,
        sign_in_count: 1
    };

    println!("user1 email: {}", user1.email);
    user1.email = String::from("sebastian1avila@gmail.com");
    println!("user1 email: {}", user1.email);

    let user2 = User {
        username: String::from("savila"),
        ..user1 // copia el resto de user1
    };

    println!("user2 email: {} username: {}", user2.email, user2.username);
}