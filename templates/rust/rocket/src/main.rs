#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "hello world!"
}

#[launch]
fn rocket() -> _ {
    let rcfg = rocket::Config {
        port: 8080,
        ..Default::default()
    };

    rocket::build().configure(&rcfg).mount("/", routes![hello])
}
