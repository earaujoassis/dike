use std::env;
use slog::*;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

#[derive(Clone)]
pub struct GetSaltMiddleware {
    pub logger: Logger,
    salt: String
}

impl GetSaltMiddleware {
    pub fn new (logger: &Logger) -> GetSaltMiddleware {
        let salt = env::var("KNOCK_PASSWORD_SALT").expect("KNOCK_PASSWORD_SALT must be set");

        GetSaltMiddleware {logger: logger.new(o!("module" => "GetSaltMiddleware")), salt: salt}
    }
}

pub struct Value(String);

impl typemap::Key for GetSaltMiddleware { type Value = Value; }

impl BeforeMiddleware for GetSaltMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<GetSaltMiddleware>(Value(self.salt.clone()));
        Ok(())
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}

pub trait GetSaltReqExt {
    fn get_salt(&self) -> &String;
}

impl <'a, 'b>GetSaltReqExt for Request <'a, 'b> {
    fn get_salt(&self) -> &String {
        let &Value(ref salt) = self.extensions.get::<GetSaltMiddleware>().unwrap();

        salt
    }
}
