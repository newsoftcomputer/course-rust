
// Structure into function

#[derive(Debug)]
struct Usuario {
    user: String,
    email: String,
    time_active: u64,
    active: bool
}


fn main() {
    let user1 = build_user(String::from("andresganc"), String::from("andresganc@gmail.com"));
    println!("{:?}", user1);
}


fn build_user(user: String, email: String) -> Usuario {
    Usuario{
        user: user,
        email,
        time_active: 1,
        active: true
    }
}