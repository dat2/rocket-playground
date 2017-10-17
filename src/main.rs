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
  id: usize,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
  type Error = String;

  fn from_request(request: &'a Request) -> Outcome<Self, Self::Error> {
    let header_map = request.headers();
    header_map.get_one("Authorization")
      .and_then(|value| value.parse::<auth::Auth>().ok())
      .and_then(|auth| {
        match auth {
          auth::Auth::Basic(auth::Basic { ref user, ref pass }) if user == "nick" && pass == "dujay" => Some(Admin{ id: 1 }),
          auth::Auth::Bearer(auth::Bearer { ref token }) if token == "123456" => Some(Admin{ id: 2 }),
          _ => None
        }
      })
      .into_outcome((Status::Unauthorized, "".to_owned()))
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
  id: usize,
  name: String,
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
