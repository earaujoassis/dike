import_controller_generic_requeriments!();

use datastore::models::client::*;

pub fn ping(_: &mut Request) -> IronResult<Response> {
    response_ok_text("pong")
}

pub fn clients(req: &mut Request) -> IronResult<Response> {
    let connection = req.get_db_conn();
    let logger = req.get_logger();

    let clients = Client::active_clients(&connection, &logger);

    response_ok(&json!({"clients": clients}))
}
