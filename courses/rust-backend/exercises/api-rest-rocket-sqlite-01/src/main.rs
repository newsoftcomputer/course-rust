
#[macro_use] extern crate rocket;

// HOME
#[get("/")]
fn index() -> &'static str {
    "Home"
}


// PROFILES
#[get("/profile")]
fn get_profile() -> &'static str {
    "Get Profile"
}


// USERS
#[get("/")]
fn get_users() -> &'static str {
    "Get Users: Geting users"
}

#[post("/")]
fn post_users() -> &'static str {
    "Post Users: Creating users"
}

#[put("/")]
fn put_users() -> &'static str {
    "Put Users: Update users"
}

#[delete("/")]
fn delete_users() -> &'static str {
    "Delete Users: Deleting users"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_profile])
    .mount("/users", routes![get_users, post_users, put_users, delete_users])
}
