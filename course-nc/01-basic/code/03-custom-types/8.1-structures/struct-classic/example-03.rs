
// Instancing all struct date Usuario with macro #[derive(Debug)] and {:?}
// Mut struct

#[derive(Debug)]
struct Usuario {
    user: String,
    email: String,
    time_active: u64,
    active: bool
}

fn main() {
    let mut user1 = Usuario{
        email: String::from("andresganc@gmail.com"),
        user: String::from("andresganc"),
        time_active: 1,
        active: true
    };

    println!("{:?}", user1);

    user1.user = String::from("andresgadev");

    println!("New value user: {} {} {} {}", user1.user, user1.email, user1.time_active, user1.active);
}