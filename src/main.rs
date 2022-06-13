#[macro_use] extern crate rocket;

use rocket::http::Header;
use rocket::response::Responder;
use rocket::serde::json::{Json};
use rocket::serde::{Serialize};

use person_registration::{Gen, Person, Juridic, General};

#[derive(Debug, PartialEq, FromFormField)]
enum Kind {
    Physical,
    Juridic,
    General,
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

#[derive(Responder)]
#[response(content_type = "application/json")]
struct Register {
    registration: Json<Registration>,
    access_control_allow_origin: Header<'static>,
}

impl Register {
    fn new(registration: Json<Registration>) -> Register {
        Register {
            registration,
            access_control_allow_origin: Header::new("Access-Control-Allow-Origin", "*"),
        }
    }
}

#[get("/")]
fn index() -> Json<Healthcheck<'static>> {
    Json(Healthcheck {
        ping: "pong"
    })
}

#[get("/?<kind>")]
fn registration(kind: Kind) -> Register {
    let output = match kind {
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
        Kind::General => {
            Json(Registration {
                data: General::generate(),
            })
        }
    };
    Register::new(output)
}

#[options("/?<kind>")]
fn registration_options(kind: Kind) -> Register {
    let output = match kind {
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
        Kind::General => {
            Json(Registration {
                data: General::generate(),
            })
        }
    };
    Register::new(output)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, registration, registration_options])
}
