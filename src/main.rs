#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate base64;
#[macro_use]
extern crate error_chain;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use rocket::Request;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{FromRequest, Outcome};
use rocket_contrib::Json;

mod auth;

#[derive(Debug)]
struct Admin {
  id: usize
}

impl<'a,'r> FromRequest<'a,'r> for Admin {
  type Error = String;

  fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
    let header_map = request.headers();
    header_map.get_one("Authorization")
      .and_then(|value| value.parse::<auth::BasicAuth>().ok())
      .and_then(|auth| {
        if &auth.user == "nick" && &auth.pass == "dujay" {
          Some(Admin { id: 1 })
        } else {
          None
        }
      })
      .into_outcome((Status::Unauthorized, "".to_owned()))
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
  id: usize,
  name: String
}

#[post("/users", format = "application/json", data = "<user>")]
fn create_user(admin: Admin, user: Json<User>) -> String {
  println!("{:?}", admin);
  format!("Created user {}!", user.id)
}

#[get("/hello/<id>")]
fn hello(id: usize) -> String {
  format!("Getting user {}", id)
}

fn main() {
  rocket::ignite()
    .mount("/api", routes![hello, create_user])
    .launch();
}
