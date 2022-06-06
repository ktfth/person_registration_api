#[macro_use] extern crate rocket;

use rocket::serde::json::{Json};
use rocket::serde::{Serialize};

use person_registration::{Gen, Person, Juridic};

#[derive(Debug, PartialEq, FromFormField)]
enum Kind {
    Physical,
    Juridic
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Healthcheck<'a> {
    ping: &'a str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Registration {
    data: String,
}

#[get("/")]
fn index() -> Json<Healthcheck<'static>> {
    Json(Healthcheck {
        ping: "pong"
    })
}

#[get("/?<kind>")]
fn registration(kind: Kind) -> Json<Registration> {
    match kind {
        Kind::Physical => {
            Json(Registration {
                data: Person::generate(),
            })
        },
        Kind::Juridic => {
            Json(Registration {
                data: Juridic::generate(),
            })
        },
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, registration])
}
