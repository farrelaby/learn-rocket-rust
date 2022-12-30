#[macro_use]
extern crate rocket;

#[get("/hello/<name>")]
fn world(name: &str) -> String {
    format!("Hello, {}! howdy!!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![world])
}
