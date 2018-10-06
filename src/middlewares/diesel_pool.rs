use slog::Logger;

use iron::{typemap, BeforeMiddleware};
use iron::prelude::*;

use utils::pool::DieselPool;
use utils::pool::DieselConnection;

pub struct DieselMiddleware {
    pool: DieselPool
}

pub struct Value(DieselPool);

impl typemap::Key for DieselMiddleware { type Value = Value; }

impl DieselMiddleware {
    pub fn new (_: &Logger, pool: &DieselPool) -> DieselMiddleware {
        DieselMiddleware {pool: pool.clone()}
    }
}

impl BeforeMiddleware for DieselMiddleware {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DieselMiddleware>(Value(self.pool.clone()));
        Ok(())
    }
}

pub trait DieselReqExt {
    fn get_db_conn(&self) -> DieselConnection;
}

impl <'a, 'b>DieselReqExt for Request <'a, 'b> {
    fn get_db_conn(&self) -> DieselConnection {
        let &Value(ref pool) = self.extensions.get::<DieselMiddleware>().unwrap();

        return pool.get().expect("Failed to get a database connection");
    }
}
