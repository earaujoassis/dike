#[allow(unused_macros)]
macro_rules! import_controller_generic_requeriments {
    ($($includes:ident),*) => {
        #[allow(unused_imports)]
        use std::error::Error;
        #[allow(unused_imports)]
        use std::io::Read;
        #[allow(unused_imports)]
        use serde_json;
        #[allow(unused_imports)]
        use serde_json::Value;
        #[allow(unused_imports)]
        use serde_json::Value::Object;
        #[allow(unused_imports)]
        use serde_json::Map;
        #[allow(unused_imports)]
        use actix_web::{HttpRequest, HttpResponse};
        #[allow(unused_imports)]
        use actix_web::http::StatusCode;

        #[allow(unused_imports)]
        use middlewares::RequestDiesel;
        #[allow(unused_imports)]
        use middlewares::RequestLogger;
        #[allow(unused_imports)]
        use middlewares::RequestSalt;
        #[allow(unused_imports)]
        use datastore::*;
        #[allow(unused_imports)]
        use datastore::models;
        #[allow(unused_imports)]
        use controllers::utils::*;
        #[allow(unused_imports)]
        use utils::macros;

        $(
            use $includes;
        )*
    }
}

#[allow(unused_macros)]
macro_rules! create_http_response {
    ($name:ident, $status:expr, "to_json_error") => {
        #[allow(dead_code)]
        pub fn $name<S: Into<String>>(text: S) -> HttpResponse {
            return HttpResponse::Ok()
                .status($status)
                .json(json!({"error": text.into()}));
        }
    };
    ($name:ident, $status:expr, "to_json") => {
        #[allow(dead_code)]
        pub fn $name(response: Value) -> HttpResponse {
            return HttpResponse::Ok()
                .status($status)
                .json(response);
        }
    };
    ($name:ident, $status:expr, "text") => {
        #[allow(dead_code)]
        pub fn $name<S: Into<String>>(text: S) -> HttpResponse {
            return HttpResponse::Ok()
                .status($status)
                .body(text.into());
        }
    };
}

#[allow(unused_macros)]
macro_rules! get_body_as {
    ($structure:ty, $req:expr, $error_fn:ident) => {
        {
            let body = get_body!($req, $error_fn);
            let structure = serde_json::from_str::<$structure>(&body);

            match structure {
                Ok(structure) => structure,
                Err(error) => return $error_fn(format!("{}: {}", error.description(), error))
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_body {
    ($req:expr, $error_fn:ident) => {
        {
            let mut payload = String::new();
            if let Err(_) = $req.body.read_to_string(&mut payload) {
                return $error_fn("Request body not found")
            }

            payload
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_route_parameter_as {
    ($parse_type:ty, $req:expr, $param:expr, $return_http:expr) => {
        {
            let ref param = get_route_parameter!($req, $param, $return_http);

            match param.parse::<$parse_type>() {
                Ok(expr) => expr,
                Err(_) => return $return_http
            }
        }
    }
}

#[allow(unused_macros)]
macro_rules! get_route_parameter {
    ($req:expr, $param:expr, $return_http:expr) => {
        {
            let param = $req.extensions.get::<Router>().unwrap().find($param);

            some_or_return!(param, $return_http)
        }
    }
}

#[allow(unused_macros)]
macro_rules! filter_struct_values_for_json {
    ($item:expr, $($key:expr),*) => {
        {
            let mut structure = serde_json::to_value(&$item).expect("Not possible to serialize to JSON");

            $(
                if let Object(ref mut object) = structure {
                    object.remove($key);
                }
            )*

            structure
        }
    }
}

#[allow(unused_imports)]
use actix_web::http::StatusCode;
#[allow(unused_imports)]
use actix_web::{HttpRequest, HttpResponse};
#[allow(unused_imports)]
use serde_json;
#[allow(unused_imports)]
use serde_json::Value;

#[allow(dead_code)]
pub fn response_text<S: Into<String>>(text: S, status:StatusCode) -> HttpResponse {
    return HttpResponse::Ok()
        .status(status)
        .body(text.into());
}

create_http_response!(response_ok, StatusCode::OK, "to_json");
create_http_response!(response_ok_text, StatusCode::OK, "text");
create_http_response!(response_not_found, StatusCode::NOT_FOUND, "to_json_error");
create_http_response!(response_bad_request, StatusCode::BAD_REQUEST, "to_json_error");
create_http_response!(response_internal_server_error, StatusCode::INTERNAL_SERVER_ERROR, "to_json_error");
