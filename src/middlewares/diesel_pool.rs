use slog::Logger;
use actix_web::{HttpRequest, Result};
use actix_web::middleware::{Middleware, Started};

use utils::pool::DieselPool;
use utils::pool::DieselConnection;

#[derive(Clone)]
pub struct DieselMiddleware {
    pool: DieselPool
}

impl DieselMiddleware {
    pub fn new (_: &Logger, pool: &DieselPool) -> DieselMiddleware {
        DieselMiddleware {pool: pool.clone()}
    }
}

impl<S> Middleware<S> for DieselMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        req.extensions_mut().insert::<DieselPool>(self.pool.clone());
        Ok(Started::Done)
    }
}

pub trait DieselReqExt {
    fn get_db_conn(&self) -> DieselConnection;
}

impl DieselReqExt for HttpRequest {
    fn get_db_conn(&self) -> DieselConnection {
        self.extensions().get::<DieselPool>()
            .unwrap()
            .get()
            .expect("Failed to get a database connection")
    }
}
