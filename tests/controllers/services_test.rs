extern crate dike;

#[cfg(test)]
mod tests {
    use std::str;
    use actix_web::{http, HttpMessage};
    use test_helper;

    #[test]
    fn test_services_index() {
        let mut server = test_helper();
        let request = server.client(http::Method::GET, "/").finish().unwrap();
        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
        let bytes = server.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "{\"message\":\"You shall not pass!\"}");
    }

    #[test]
    fn test_services_ping() {
        let mut server = test_helper();
        let request = server.client(http::Method::GET, "/ping").finish().unwrap();
        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
        let bytes = server.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "{\"message\":\"pong\"}");
    }

    #[test]
    fn test_services_clients_active() {
        let mut server = test_helper();
        let request = server.client(http::Method::GET, "/clients/active").finish().unwrap();
        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
        let bytes = server.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "{\"clients\":[]}");
    }

    #[test]
    fn test_services_clients_update() {
        let mut server = test_helper();
        let request = server.client(http::Method::POST, "/clients/update").finish().unwrap();
        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
        let bytes = server.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "{\"clients\":null}");
    }
}
