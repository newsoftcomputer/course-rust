
// Instancing all struct date Usuario with macro #[derive(Debug)] and {:?}

#[derive(Debug)]
struct Usuario {
    user: String,
    email: String,
    time_ctive: u64,
    active: bool
}

fn main() {
    let user1 = Usuario{
        email: String::from("andresganc@gmail.com"),
        user: String::from("andresganc"),
        time_ctive: 1,
        active: true
    };

    println!("{:?}", user1);
}