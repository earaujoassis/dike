use std::env;
use slog::*;
use actix_web::{HttpRequest, Result};
use actix_web::middleware::{Middleware, Started};

#[derive(Clone)]
pub struct GetSaltMiddleware {
    pub logger: Logger,
    salt: String
}

impl GetSaltMiddleware {
    pub fn new (logger: &Logger) -> GetSaltMiddleware {
        let salt = env::var("DIKE_PASSWORD_SALT").expect("DIKE_PASSWORD_SALT must be set");

        GetSaltMiddleware {logger: logger.new(o!("module" => "GetSaltMiddleware")), salt: salt}
    }
}

impl<S> Middleware<S> for GetSaltMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.extensions_mut().insert::<GetSaltMiddleware>(self.clone());
        Ok(Started::Done)
    }
}

pub trait GetSaltReqExt {
    fn get_salt(&self) -> String;
}

impl GetSaltReqExt for HttpRequest {
    fn get_salt(&self) -> String {
        self.extensions().get::<GetSaltMiddleware>().unwrap().salt.clone()
    }
}
