import_controller_generic_requeriments!();

pub fn index(_req: &HttpRequest) -> HttpResponse {
    response_ok_text("You shall not pass!")
}

pub fn ping(_req: &HttpRequest) -> HttpResponse {
    response_ok_text("pong")
}

pub fn clients(req: &HttpRequest) -> HttpResponse {
    let connection = req.get_db_conn();
    let logger = req.get_logger();
    response_ok(json!({"clients": client::Client::active_clients(&connection, &logger)}))
}

#[cfg(test)]
mod tests {
    use actix_web::{test, http};
    use actix_web::Body;
    use super::*;

    #[test]
    fn test_index() {
        let response = test::TestRequest::default().run(&index).unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(response.body(), &Body::from(String::from("You shall not pass!").into_bytes()));
    }

    #[test]
    fn test_ping() {
        let response = test::TestRequest::default().run(&ping).unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
        assert_eq!(response.body(), &Body::from(String::from("pong").into_bytes()));
    }
}
