
#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
use rocket::serde::{Serialize, json::Json};

// STRUCTS
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Task { /* .. */ }

// HOME
// #[get("/")]
// fn index() -> &'static str {
//     "Home"
// }

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { title: "Rocket SQLite"})
}


// PROFILES
#[get("/profile")]
fn get_profile() -> &'static str {
    "Get Profile"
}

// TODO
#[get("/todo")]
fn todo() -> Json<Task> {
    Json(Task { /* .. */ })
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
    .attach(Template::fairing())
}

