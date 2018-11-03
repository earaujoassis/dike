use iron::prelude::*;
use slog::Logger;

use http_adaptor::general_endpoints;
use utils::pool::DieselPool;

pub struct HttpAdaptor {
    logger: Logger,
    pool: DieselPool
}

impl HttpAdaptor {
    pub fn new(logger: &Logger, pool: &DieselPool) -> HttpAdaptor {
        HttpAdaptor {logger: logger.new(o!("module" => "HttpAdaptor")), pool: pool.clone()}
    }

    pub fn start_http(&self, host: &str, port: &str) {
        let routes = general_endpoints(&self.logger, &self.pool);
        let address = format!("{}:{}", host, port);

        {
            info!(self.logger, "Server Running"; o!("address" => address.clone()));
        }

        Iron::new(routes).http(address).unwrap();
    }
}
