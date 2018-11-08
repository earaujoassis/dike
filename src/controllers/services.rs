import_controller_generic_requeriments!();

pub fn index(_req: &HttpRequest) -> HttpResponse {
    response_ok(json!({"message": "You shall not pass!"}))
}

pub fn ping(_req: &HttpRequest) -> HttpResponse {
    response_ok(json!({"message": "pong"}))
}

pub fn clients_update(_req: &HttpRequest) -> HttpResponse {
    response_ok(json!({"clients": Value::Null}))
}

pub fn clients_active(req: &HttpRequest) -> HttpResponse {
    let connection = req.datastore();
    let logger = req.logger();
    response_ok(json!({"clients": client::Client::active_clients(&connection, &logger)}))
}
