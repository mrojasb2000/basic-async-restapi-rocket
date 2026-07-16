// Here, we are telling the Rust compiler to use macros from Rocket crate
#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, Rocket!"
}

#[rocket::main]
async fn main() {
    let _ = rocket::build().mount("/", routes![index])
        .launch()
        .await;
}