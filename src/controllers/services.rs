import_controller_generic_requeriments!();

pub fn index(_req: &HttpRequest) -> HttpResponse {
    response_ok_text("You shall not pass!")
}

pub fn clients(req: &HttpRequest) -> HttpResponse {
    let connection = req.get_db_conn();
    let logger = req.get_logger();
    response_ok(json!({"clients": client::Client::active_clients(&connection, &logger)}))
}

pub fn ping(_req: &HttpRequest) -> HttpResponse {
    response_ok_text("pong")
}
