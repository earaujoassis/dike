use super::data::Client;
#[allow(unused)]
use rocket_contrib::{Json, Value};

#[get("/")]
pub fn index() -> &'static str {
    "Hello, clients!"
}

#[post("/knock", data="<client>")]
pub fn knock(client: Json<Client>) -> Json<Client> {
    let update = Client { ..client.into_inner() };
    Json(update)
}
