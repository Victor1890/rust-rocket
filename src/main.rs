#[macro_use] extern crate rocket;

use rocket::http::*;
use rocket::response::*;

#[get("/")]
fn hello_world() -> status::Custom<content::RawJson<&'static str>>{
    status::Custom(Status::ImATeapot, content::RawJson("{ \"hi\": \"world\" }"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello_world])
}
