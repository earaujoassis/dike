use slog::*;
use actix_web::{HttpRequest, Result};
use actix_web::middleware::{Middleware, Started};

#[derive(Clone)]
pub struct LoggerMiddleware {
    pub logger: Logger
}

impl LoggerMiddleware {
    pub fn new (logger: &Logger) -> LoggerMiddleware {
        LoggerMiddleware {logger: logger.new(o!("module" => "LoggerMiddleware"))}
    }
}

impl<S> Middleware<S> for LoggerMiddleware {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let logger = self.logger.new(o!("path" => format!("{}", req.path()), "method" => format!("{}", req.method())));
        info!(logger, "Path reached");
        req.extensions_mut().insert::<Logger>(logger);
        Ok(Started::Done)
    }
}

pub trait RequestLogger {
    fn logger(&self) -> Logger;
}

impl RequestLogger for HttpRequest {
    fn logger(&self) -> Logger {
        self.extensions().get::<Logger>().unwrap().clone()
    }
}
