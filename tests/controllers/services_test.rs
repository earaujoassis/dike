extern crate knock;

#[cfg(test)]
mod tests {
    use std::str;
    use actix_web::{http, HttpMessage};
    use test_helper;

    #[test]
    fn test_services_clients() {
        let mut server = test_helper();
        let request = server.client(http::Method::GET, "/clients").finish().unwrap();
        let response = server.execute(request.send()).unwrap();
        assert!(response.status().is_success());
        let bytes = server.execute(response.body()).unwrap();
        let body = str::from_utf8(&bytes).unwrap();
        assert_eq!(body, "{\"clients\":[]}");
    }
}
